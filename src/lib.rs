use core::ops::Deref;

use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
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
}

impl Deref for ClientId {
    type Target = Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

#[init]
fn init(_: RString) -> State {
    State {
        clients: Clients::get()
            .expect("Failed to get clients")
            .iter()
            .filter(|client| !(client.title.is_empty() && client.class.is_empty()))
            .enumerate()
            .map(|(idx, client)| ClientId {
                id: idx as u64,
                search: format!("{}: {}", client.class, client.title),
                client: client.clone(),
            })
            .collect(),
    }
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
    state
        .clients
        .iter()
        .filter(|c| !(c.title.is_empty() && c.class.is_empty()))
        .filter(|client| matcher.fuzzy_match(&client.search, &input).is_some())
        .map(|client| Match {
            title: client.class.clone().into(),
            icon: ROption::RSome(icon_from_class(&client.class).into()),
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

fn icon_from_class(class: impl AsRef<str>) -> String {
    let class = class.as_ref().to_lowercase();
    if class.contains('.') {
        class.split('.').last().unwrap_or_default().into()
    } else {
        class
    }
}
