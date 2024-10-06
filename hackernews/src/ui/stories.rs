use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::hooks::use_resource;
use dioxus::prelude::Readable;
use crate::story::get_top_stories;
use crate::ui::story_item::StoryItem;

#[component]
fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));

    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                for story in stories {
                    StoryItem {
                        story: story.clone()
                    }
                }
            }
        },
        Some(Err(err)) => rsx! {
            div {
                class: "mt-6 text-red-500",
                p { "Failed to fetch stories" }
                p { "{err}" }
            }
        },
        None => rsx! {
            div {
                class: "mt-6",
                p { "Loading stories" }
            }
        }
    }
}