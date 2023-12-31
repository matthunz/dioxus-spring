use std::time::Duration;

use dioxus::prelude::*;
use dioxus_spring::{use_animated, use_spring};
use dioxus_use_mounted::use_mounted;
use log::LevelFilter;

fn app(cx: Scope) -> Element {
    let is_big = use_state(cx, || false);
    let spring = use_spring(
        cx,
        if **is_big { 2f32 } else { 1f32 },
        Duration::from_millis(500),
    );

    let mounted = use_mounted(cx);
    use_animated(cx, mounted, spring, |scale| {
        format!("transform-origin: top left; transform: scale({scale})")
    });

    log::info!("render!");

    render!(
        div {
            onmounted: move |event| mounted.onmounted(event),
            onclick: move |_| is_big.set(!is_big),
            "Click me!"
        }
    )
}

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
