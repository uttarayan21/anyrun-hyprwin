use core::ops::Deref;
use std::collections::HashMap;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use error_stack::ResultExt;
use hyprland::data::{Client, Clients};
use hyprland::shared::HyprData;

#[derive(Debug, Clone)]
pub struct ClientId {
    pub client: Client,
    pub search: String,
    pub id: u64,
}

pub struct State {
    pub clients: Vec<ClientId>,
    pub config: Config,
}

impl Deref for ClientId {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[derive(Debug, Clone, thiserror::Error)]
#[error("anyrun: hyprwin plugin error")]
pub struct HyprwinError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    prefix: String,
    #[serde(default)]
    icons: HashMap<String, String>,
}

impl Config {
    pub fn from_str(s: impl AsRef<str>) -> error_stack::Result<Self, HyprwinError> {
        ron::de::from_str(s.as_ref())
            .change_context(HyprwinError)
            .attach_printable("Failed to parse config file for hyprwin")
    }
    pub fn from_path(p: impl AsRef<std::path::Path>) -> error_stack::Result<Self, HyprwinError> {
        if !p.as_ref().exists() {
            tracing::info!("No config file found for hyprwin");
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(p.as_ref())
            .change_context(HyprwinError)
            .attach_printable("Failed to read config file for hyprwin")?;
        Self::from_str(contents)
    }
}

#[init]
fn init(config: RString) -> State {
    init_result(config).expect("Failed to initialize hyprwin")
}

fn init_result(config: RString) -> error_stack::Result<State, HyprwinError> {
    let config_path = std::path::Path::new(config.as_str()).join("hyprwin.ron");
    let config = Config::from_path(config_path)?;
    Ok(State {
        clients: Clients::get()
            .change_context(HyprwinError)
            .attach_printable("Failed to get clients")?
            .iter()
            .filter(|client| !(client.title.is_empty() && client.class.is_empty()))
            .enumerate()
            .map(|(idx, client)| ClientId {
                id: idx as u64,
                search: format!("{}: {}", client.class, client.title),
                client: client.clone(),
            })
            .collect(),
        config,
    })
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Hyprland Windows".into(),
        icon: "window-new".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, state: &State) -> RVec<Match> {
    use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
    let matcher = SkimMatcherV2::default();
    let prefix = state.config.prefix.clone();
    let input = if !input.starts_with(&prefix) {
        return RVec::new();
    } else {
        input
            .strip_prefix(&prefix)
            .expect("UNEXPECTED: Should not fail please report this")
            .to_string()
    };
    state
        .clients
        .iter()
        .filter(|c| !(c.title.is_empty() && c.class.is_empty()))
        .filter(|client| matcher.fuzzy_match(&client.search, &input).is_some())
        .map(|client| Match {
            title: client.class.clone().into(),
            icon: ROption::RSome(icon_from_class(&client.class, &state.config.icons).into()),
            use_pango: false,
            description: ROption::RSome(client.title.clone().into()),
            id: ROption::RSome(client.id),
        })
        .collect::<Vec<Match>>()
        .into()
}

#[handler]
fn handler(selection: Match, state: &State) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    use hyprland::dispatch::*;
    let Some(address) = state
        .clients
        .iter()
        .find(|c| c.id == selection.id.unwrap_or_default())
        .map(|c| c.address.clone())
    else {
        return HandleResult::Close;
    };
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::Address(
        address,
    )))
    .expect("Unable to focus hyprland window");
    HandleResult::Close
}

fn icon_from_class(class: impl AsRef<str>, icons: &HashMap<String, String>) -> String {
    let class = class.as_ref().to_lowercase();
    if let Some(icon) = icons.get(&class) {
        icon.clone()
    } else if class.contains('.') {
        class.split('.').last().unwrap_or_default().into()
    } else {
        class
    }
}
