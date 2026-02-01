use dioxus::prelude::*;
use logger::info;
use crate::logger;

// Token Value Saving
#[component]
pub fn MainPage() -> Element {
    info("MainPage", "Loaded Main Page");
    rsx! {
        div {
            class: "apptitle text-shadow-indigo-200",
            h1 {"RimaTon"}
        }
        div {
            id: "appmoto",
            "The Exaroton Client"
        }
        div {
            id: "app_login_text",
            "Please Login to Exaroton Via API Token"
        }
        input {
            placeholder: "API Token (Treat That As Password)",
            type: "password",
            oninput: move |exaroton_api_input| {
                let mut exaroton_api = use_signal(|| {exaroton_api_input}.to_owned());
                println!("Token Added")
            }

        }
    }
}