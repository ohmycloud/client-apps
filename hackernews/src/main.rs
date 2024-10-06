#![allow(non_snake_case)]

mod story;
mod ui;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use crate::ui::App;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}
