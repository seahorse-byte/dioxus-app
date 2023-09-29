#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Dioxus world!" }
        h4 { "My first heading alright??" }
        footer { class: "info",
        br {}
        hr {}
        p { "Created by ", a {  href: "https://umuv.world", target: "_blank", "Olsi Gjeci" }}
        p { "With ", a { href: "https://dioxuslabs.com/", "Dioxus" }}
        hr {}
        button {class: "btn", "Dioxus"}
    }
    })
}
