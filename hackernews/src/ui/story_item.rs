#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::story::StoryItem;

#[component]
pub fn StoryItem(story: StoryItem) -> Element {
    let title = story.title;
    rsx! {
        li { class: "px-3 py-5 transition border-b hover:bg-indigo-100",
            a { href: "#", class: "flex items-center justify-between",
                h3 { class: "text-lg font-semibold", "{title}" }
                p { class: "text-gray-400 text-md" }
            }
            div { class: "italic text-gray-400 text-md",
                span { "{story.score} points by {story.by} {story.time} | " }
                a { href: "#", "{story.kids.len()} comments" }
            }
       }
    }
}