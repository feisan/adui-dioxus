//! Image component demo.

use adui_dioxus::components::image::{
    Image, ImagePreviewGroup, ImagePreviewItem, PreviewConfig,
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
                h1 { "Image Component" }
                p { "An image component with preview support." }

                h2 { "Basic Usage" }
                BasicImage {}

                h2 { "With Fallback" }
                FallbackImage {}

                h2 { "Preview Disabled" }
                NoPreviewImage {}

                h2 { "Custom Preview Mask" }
                CustomMaskImage {}

                h2 { "Image Gallery" }
                ImageGallery {}

                h2 { "Preview Group" }
                PreviewGroupDemo {}
            }
        }
    }
}

#[component]
fn BasicImage() -> Element {
    rsx! {
        div { style: "margin-bottom: 24px;",
            Image {
                src: "https://picsum.photos/300/200".to_string(),
                alt: Some("Random image".to_string()),
                width: Some("300px".to_string()),
                height: Some("200px".to_string()),
            }
        }
    }
}

#[component]
fn FallbackImage() -> Element {
    rsx! {
        div { style: "margin-bottom: 24px; display: flex; gap: 16px;",
            div {
                p { "Invalid source with fallback:" }
                Image {
                    src: "https://invalid-url-that-will-fail.jpg".to_string(),
                    fallback: Some("https://picsum.photos/150/150".to_string()),
                    width: Some("150px".to_string()),
                    height: Some("150px".to_string()),
                }
            }
            div {
                p { "Both sources invalid:" }
                Image {
                    src: "https://invalid-url-1.jpg".to_string(),
                    fallback: Some("https://invalid-url-2.jpg".to_string()),
                    width: Some("150px".to_string()),
                    height: Some("150px".to_string()),
                }
            }
        }
    }
}

#[component]
fn NoPreviewImage() -> Element {
    rsx! {
        div { style: "margin-bottom: 24px;",
            Image {
                src: "https://picsum.photos/200/150".to_string(),
                alt: Some("No preview".to_string()),
                width: Some("200px".to_string()),
                height: Some("150px".to_string()),
                preview: false,
            }
        }
    }
}

#[component]
fn CustomMaskImage() -> Element {
    let config = PreviewConfig::new().with_mask("Click to view");

    rsx! {
        div { style: "margin-bottom: 24px;",
            Image {
                src: "https://picsum.photos/250/180".to_string(),
                alt: Some("Custom mask".to_string()),
                width: Some("250px".to_string()),
                height: Some("180px".to_string()),
                preview_config: Some(config),
            }
        }
    }
}

#[component]
fn ImageGallery() -> Element {
    let images = [
        ("https://picsum.photos/200/200?random=1", "Image 1"),
        ("https://picsum.photos/200/200?random=2", "Image 2"),
        ("https://picsum.photos/200/200?random=3", "Image 3"),
        ("https://picsum.photos/200/200?random=4", "Image 4"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; display: flex; flex-wrap: wrap; gap: 12px;",
            for (src, alt) in images {
                Image {
                    src: src.to_string(),
                    alt: Some(alt.to_string()),
                    width: Some("150px".to_string()),
                    height: Some("150px".to_string()),
                }
            }
        }
    }
}

#[component]
fn PreviewGroupDemo() -> Element {
    let mut visible: Signal<bool> = use_signal(|| false);
    let mut current: Signal<usize> = use_signal(|| 0);

    let items = vec![
        ImagePreviewItem::new("https://picsum.photos/800/600?random=10")
            .with_alt("Large Image 1"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=11")
            .with_alt("Large Image 2"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=12")
            .with_alt("Large Image 3"),
        ImagePreviewItem::new("https://picsum.photos/800/600?random=13")
            .with_alt("Large Image 4"),
    ];

    let thumbnails = [
        "https://picsum.photos/150/150?random=10",
        "https://picsum.photos/150/150?random=11",
        "https://picsum.photos/150/150?random=12",
        "https://picsum.photos/150/150?random=13",
    ];

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Click any thumbnail to open the group preview:" }
            div { style: "display: flex; gap: 8px;",
                for (i, thumb) in thumbnails.iter().enumerate() {
                    img {
                        src: "{thumb}",
                        style: "width: 100px; height: 100px; object-fit: cover; cursor: pointer; border-radius: 4px;",
                        onclick: move |_| {
                            current.set(i);
                            visible.set(true);
                        },
                    }
                }
            }

            ImagePreviewGroup {
                items: items.clone(),
                visible: *visible.read(),
                current: *current.read(),
                on_visible_change: Some(EventHandler::new(move |v| visible.set(v))),
                on_change: Some(EventHandler::new(move |idx| current.set(idx))),
            }
        }
    }
}
