//! Mentions component demo.

use adui_dioxus::components::config_provider::ComponentSize;
use adui_dioxus::components::mentions::{MentionOption, MentionPlacement, Mentions, get_mentions};
use adui_dioxus::theme::ThemeProvider;
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            div { style: "padding: 24px; max-width: 600px; margin: 0 auto;",
                h1 { "Mentions Component" }
                p { "An input component that supports @-style mentions." }

                h2 { "Basic Usage" }
                BasicMentions {}

                h2 { "Multiple Prefixes" }
                MultiplePrefixMentions {}

                h2 { "Different Sizes" }
                SizedMentions {}

                h2 { "Placement Top" }
                TopPlacementMentions {}

                h2 { "Parse Mentions" }
                ParseMentionsDemo {}
            }
        }
    }
}

#[component]
fn BasicMentions() -> Element {
    let options = vec![
        MentionOption::new("john", "John Doe"),
        MentionOption::new("jane", "Jane Smith"),
        MentionOption::new("bob", "Bob Wilson"),
        MentionOption::new("alice", "Alice Brown"),
    ];

    let mut value: Signal<String> = use_signal(String::new);

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Type @ to mention someone:" }
            Mentions {
                value: value.read().clone(),
                options: options,
                placeholder: "Type @ to mention...",
                rows: 3,
                on_change: move |v: String| value.set(v),
            }
            p { style: "margin-top: 8px; color: #666; font-size: 12px;",
                "Value: {value.read()}"
            }
        }
    }
}

#[component]
fn MultiplePrefixMentions() -> Element {
    let options = vec![
        MentionOption::new("react", "React"),
        MentionOption::new("vue", "Vue"),
        MentionOption::new("angular", "Angular"),
        MentionOption::new("svelte", "Svelte"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Use @ for mentions or # for tags:" }
            Mentions {
                options: options,
                prefix: vec!['@', '#'],
                placeholder: "Type @ or # ...",
                rows: 3,
            }
        }
    }
}

#[component]
fn SizedMentions() -> Element {
    let options = vec![
        MentionOption::simple("option1"),
        MentionOption::simple("option2"),
        MentionOption::simple("option3"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px; display: flex; flex-direction: column; gap: 12px;",
            Mentions {
                options: options.clone(),
                size: ComponentSize::Small,
                placeholder: "Small size...",
            }
            Mentions {
                options: options.clone(),
                size: ComponentSize::Middle,
                placeholder: "Middle size (default)...",
            }
            Mentions {
                options: options.clone(),
                size: ComponentSize::Large,
                placeholder: "Large size...",
            }
        }
    }
}

#[component]
fn TopPlacementMentions() -> Element {
    let options = vec![
        MentionOption::new("north", "North"),
        MentionOption::new("south", "South"),
        MentionOption::new("east", "East"),
        MentionOption::new("west", "West"),
    ];

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Dropdown opens above the input:" }
            div { style: "margin-top: 120px;",
                Mentions {
                    options: options,
                    placement: MentionPlacement::Top,
                    placeholder: "Type @ ...",
                    rows: 2,
                }
            }
        }
    }
}

#[component]
fn ParseMentionsDemo() -> Element {
    let mut text: Signal<String> = use_signal(|| "@john hello @alice how are you #project".into());

    let mentions = get_mentions(&text.read(), &['@', '#']);

    rsx! {
        div { style: "margin-bottom: 24px;",
            p { "Parse mentions from text:" }
            textarea {
                style: "width: 100%; min-height: 60px; padding: 8px; border: 1px solid #d9d9d9; border-radius: 4px;",
                value: "{text.read()}",
                oninput: move |evt| text.set(evt.value()),
            }
            div { style: "margin-top: 12px; padding: 12px; background: #f5f5f5; border-radius: 4px;",
                h4 { style: "margin: 0 0 8px 0;", "Parsed Mentions:" }
                if mentions.is_empty() {
                    p { style: "color: #999;", "No mentions found" }
                } else {
                    ul { style: "margin: 0; padding-left: 20px;",
                        for mention in mentions {
                            li { "{mention.prefix}{mention.value}" }
                        }
                    }
                }
            }
        }
    }
}
