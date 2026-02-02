mod mainpage;
mod logger;
mod configuration;

use logger::info;
use dioxus::prelude::*;
use dioxus::router::Routable;

#[cfg(not(target_arch = "wasm32"))]
use configuration::{config_path, check_file};
use mainpage::MainPage;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    info("Main", "Initializing");

    // Native only (Linux / Windows / macOS)
    #[cfg(not(target_arch = "wasm32"))]
    {
        info("Main", "Config File Checking...");

        match config_path() {
            Ok(path) => check_file(&path),
            Err(e) => logger::err("Config", &e.to_string(), 1),
        }
    }

    // WASM-safe
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    MP {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn MP() -> Element {
    MainPage()
}

/* fn LoginP() -> Element {
    
}
*/
