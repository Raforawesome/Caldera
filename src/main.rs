#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("Caldera loaded");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "tailwind.css" }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
