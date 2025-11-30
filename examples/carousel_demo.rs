//! Carousel component demo.

use adui_dioxus::components::carousel::{
    Carousel, CarouselEffect, CarouselItem, DotPlacement,
};
use adui_dioxus::theme::ThemeProvider;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            div { style: "padding: 24px; max-width: 800px; margin: 0 auto;",
                h1 { "Carousel Component" }
                p { "A slideshow component for cycling through images or content." }

                h2 { "Basic Usage" }
                BasicCarousel {}

                h2 { "With Arrows" }
                ArrowCarousel {}

                h2 { "Fade Effect" }
                FadeCarousel {}

                h2 { "Vertical Dots" }
                VerticalDotsCarousel {}
            }
        }
    }
}

#[component]
fn BasicCarousel() -> Element {
    let items = vec![
        CarouselItem::new("Slide 1").with_background("#364d79"),
        CarouselItem::new("Slide 2").with_background("#3d5a80"),
        CarouselItem::new("Slide 3").with_background("#456990"),
        CarouselItem::new("Slide 4").with_background("#4e7da6"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; border-radius: 8px; overflow: hidden;",
            Carousel {
                items: items,
                on_change: move |index| {
                    web_sys::console::log_1(&format!("Slide changed to: {}", index).into());
                },
            }
        }
    }
}

#[component]
fn ArrowCarousel() -> Element {
    let items = vec![
        CarouselItem::new("🍎 Apple").with_background("#4a7c59"),
        CarouselItem::new("🍌 Banana").with_background("#5a8c69"),
        CarouselItem::new("🍒 Cherry").with_background("#6a9c79"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; border-radius: 8px; overflow: hidden;",
            Carousel {
                items: items,
                arrows: true,
            }
        }
    }
}

#[component]
fn FadeCarousel() -> Element {
    let items = vec![
        CarouselItem::new("Fade Slide 1").with_background("#7c4a6c"),
        CarouselItem::new("Fade Slide 2").with_background("#8c5a7c"),
        CarouselItem::new("Fade Slide 3").with_background("#9c6a8c"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; border-radius: 8px; overflow: hidden;",
            Carousel {
                items: items,
                effect: CarouselEffect::Fade,
                arrows: true,
            }
        }
    }
}

#[component]
fn VerticalDotsCarousel() -> Element {
    let items = vec![
        CarouselItem::new("Vertical Dots 1").with_background("#6c4a7c"),
        CarouselItem::new("Vertical Dots 2").with_background("#7c5a8c"),
        CarouselItem::new("Vertical Dots 3").with_background("#8c6a9c"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; border-radius: 8px; overflow: hidden;",
            Carousel {
                items: items,
                dot_placement: DotPlacement::Right,
                arrows: true,
            }
        }
    }
}
