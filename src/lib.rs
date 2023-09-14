use abi_stable::std_types::{ROption, RString, RVec};
use anyrun_plugin::*;
use hyprland::data::Clients;
use hyprland::shared::HyprData;

#[init]
fn init(_: RString) -> Clients {
    Clients::get().expect("Failed to get clients")
}

#[info]
fn info() -> PluginInfo {
    PluginInfo {
        name: "Hyprland Windows".into(),
        icon: "window-new".into(),
    }
}

#[get_matches]
fn get_matches(input: RString, clients: &Clients) -> RVec<Match> {
    use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
    let matcher = SkimMatcherV2::default();
    clients
        .clone()
        .filter(|c| !(c.title.is_empty() && c.class.is_empty()))
        .map(|mut client| {
            client.title = format!("{}: {}", client.class, client.title);
            client
        })
        .filter(|client| matcher.fuzzy_match(&client.title, &input).is_some())
        .map(|client| {
            Match {
                title: client.class.clone().into(),
                icon: ROption::RSome(icon_from_class(client.class).into()),
                use_pango: false,
                description: ROption::RSome(client.title.into()),
                id: ROption::RSome(client.pid as u64), // The ID can be used for identifying the match later, is not required
            }
        })
        .collect::<Vec<Match>>()
        .into()
}

#[handler]
fn handler(selection: Match) -> HandleResult {
    // Handle the selected match and return how anyrun should proceed
    use hyprland::dispatch::*;
    Dispatch::call(DispatchType::FocusWindow(WindowIdentifier::ProcessId(
        selection.id.unwrap_or_default() as u32,
    )))
    .expect("Unable to focus hyprland window");
    HandleResult::Close
}

fn icon_from_class(class: String) -> String {
    let class = class.to_lowercase();
    if class.contains('.') {
        class.split('.').last().unwrap_or_default().into()
    } else {
        class
    }
}
