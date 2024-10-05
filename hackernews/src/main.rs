#![allow(non_snake_case)]

mod story;
mod ui;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use crate::ui::App;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}


#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-3xl p-4", "Blog post {id}" }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-3xl font-bold mb-4 p-4",
                "High-Five counter: {count}"
            }
            button {
                class: "text-white bg-gradient-to-br from-green-400 to-blue-600 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-green-200 dark:focus:ring-green-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                onclick: move |_| count += 1, "Up high!"
            }
            button {
                class: "text-white bg-gradient-to-br from-pink-500 to-orange-400 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-pink-200 dark:focus:ring-pink-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                onclick: move |_| count -= 1, "Down low!"
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-3xl p-4", "Not Found: The page `/{path}` is not found" }
        }
    }
}
