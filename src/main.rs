use dioxus::prelude::*;
use ui::components::navbar::Navbar;
use ui::pages::{account::Account, home::Home, login::Login, register::Register};
use ui::state::app::AppState;

mod domain;
mod services;
mod ui;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/login")]
        Login {},

        #[route("/register")]
        Register {},

        #[route("/account")]
        Account {},
}

const MAIN_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // let api_base = "https://watchme-cyril-cf3191-9segqxl7.leapcell.dev";
    let api_base = "http://localhost:8080";
    let state = use_signal(|| AppState::new(api_base));

    use_context_provider(|| state);
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: MAIN_CSS,
        }

        Router::<Route> {}
    }
}
