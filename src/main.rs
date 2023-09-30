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
        div { 
            class: "flex flex-col items-center justify-center h-screen p-20 text-slate-800 bg-teal-500",
            h1 { class: "text-white text-3xl", "Hello Dioxus world!" }
            h4 { "My first heading alright??" }
            footer { class: "info"}
            br {}
            hr {}
            p { "Created by ", a {  href: "https://umuv.world", target: "_blank", "Olsi Gjeci" }}
            p { "With ", a { href: "https://dioxuslabs.com/", target: "_blank", "Dioxus" }}
            hr {}
            hr {}
            img {
                src: "https://avatars.githubusercontent.com/u/75344745?v=4",            
                class: "rounded-full h-20 w-20 border-2 border-white py-6 ",
                width: "100px"
            }
            button {class: "bg-slate-900 text-white cursor-pointer font-bold p-2 px-4 mt-10 rounded-full", "Dioxus"}
        }   
    })
}



