#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div { 
            class: "flex flex-col items-center justify-center h-screen p-20 text-slate-800 bg-teal-500",
            h1 { class: "text-white text-3xl", "Hello Dioxus world!" }
            p { "Created by ", a {  href: "https://umuv.world", target: "_blank", "Olsi Gjeci" }}
            p { "w/ ", a {class: "font-bold cursor-pointer underline", href: "https://dioxuslabs.com/", target: "_blank", "Dioxus" }}
            br {}
            br {}
            img {
                src: "https://avatars.githubusercontent.com/u/75344745?v=4",            
                class: "rounded-full border-2 border-white",
                width: "100px"
            }
            button {class: "bg-slate-900 text-white cursor-pointer font-bold p-2 px-4 mt-10 rounded-full", "Dioxus"}
            button {
                onclick: move |_| { count += 1 },
                class: "bg-slate-900 text-white cursor-pointer font-bold p-2 px-4 mt-10 rounded-full", 
                "Star me"}
                
            br {}
            p { "⭐ {count}" }
        }   
    })
}



