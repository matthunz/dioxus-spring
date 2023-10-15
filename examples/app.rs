use dioxus::prelude::*;
use dioxus_spring::use_spring;
use std::time::Duration;

fn app(cx: Scope) -> Element {
    let spring_ref = use_spring(cx, 10f32, 100f32, Duration::from_secs(1), |font_size| {
        format!("font-size: {font_size}px;")
    });

    render!(
        h1 { onmounted: move |event| {
                spring_ref.mount(event.data);
            }, "Hello World!" }
    )
}

fn main() {
    dioxus_web::launch(app)
}
