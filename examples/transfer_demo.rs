//! Transfer component demo.

use adui_dioxus::components::transfer::{Transfer, TransferDirection, TransferItem};
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
                h1 { "Transfer Component" }
                p { "A double-column transfer component for moving items between two lists." }

                h2 { "Basic Usage" }
                BasicTransfer {}

                h2 { "With Search" }
                SearchableTransfer {}

                h2 { "One-Way Mode" }
                OneWayTransfer {}

                h2 { "With Descriptions" }
                DescriptionTransfer {}
            }
        }
    }
}

#[component]
fn BasicTransfer() -> Element {
    let data_source = vec![
        TransferItem::new("1", "Item 1"),
        TransferItem::new("2", "Item 2"),
        TransferItem::new("3", "Item 3"),
        TransferItem::new("4", "Item 4"),
        TransferItem::new("5", "Item 5"),
    ];

    let mut target_keys: Signal<Vec<String>> = use_signal(|| vec!["3".into()]);

    rsx! {
        div { style: "margin-bottom: 24px;",
            Transfer {
                data_source: data_source,
                target_keys: target_keys.read().clone(),
                titles: ("Source".into(), "Target".into()),
                on_change: move |(keys, _direction, _moved): (Vec<String>, TransferDirection, Vec<String>)| {
                    target_keys.set(keys.clone());
                },
            }
        }
    }
}

#[component]
fn SearchableTransfer() -> Element {
    let data_source: Vec<TransferItem> = (1..=10)
        .map(|i| TransferItem::new(format!("{}", i), format!("Content {}", i)))
        .collect();

    let mut target_keys: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        div { style: "margin-bottom: 24px;",
            Transfer {
                data_source: data_source,
                target_keys: target_keys.read().clone(),
                show_search: true,
                search_placeholder: Some("Search items...".to_string()),
                on_change: move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                    target_keys.set(keys);
                },
            }
        }
    }
}

#[component]
fn OneWayTransfer() -> Element {
    let data_source = vec![
        TransferItem::new("a", "Apple"),
        TransferItem::new("b", "Banana"),
        TransferItem::new("c", "Cherry"),
        TransferItem::new("d", "Date"),
    ];

    let mut target_keys: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Items can only be moved from left to right." }
            Transfer {
                data_source: data_source,
                target_keys: target_keys.read().clone(),
                one_way: true,
                titles: ("Available".into(), "Selected".into()),
                operations: ("Add →".into(), "".into()),
                on_change: move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                    target_keys.set(keys);
                },
            }
        }
    }
}

#[component]
fn DescriptionTransfer() -> Element {
    let data_source = vec![
        TransferItem::new("react", "React")
            .with_description("A JavaScript library for building UIs"),
        TransferItem::new("vue", "Vue")
            .with_description("The progressive JavaScript framework"),
        TransferItem::new("angular", "Angular")
            .with_description("Platform for building mobile & desktop apps"),
        TransferItem::new("svelte", "Svelte")
            .with_description("Cybernetically enhanced web apps"),
        TransferItem::new("solid", "Solid").with_disabled(true),
    ];

    let mut target_keys: Signal<Vec<String>> = use_signal(|| vec!["react".into()]);

    rsx! {
        div { style: "margin-bottom: 24px;",
            Transfer {
                data_source: data_source,
                target_keys: target_keys.read().clone(),
                show_search: true,
                titles: ("Frameworks".into(), "My Stack".into()),
                on_change: move |(keys, _, _): (Vec<String>, TransferDirection, Vec<String>)| {
                    target_keys.set(keys);
                },
            }
        }
    }
}
