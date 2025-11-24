use adui_dioxus::{
    Button, ButtonShape, ButtonSize, ButtonType, Theme, ThemeMode, ThemeProvider, use_theme,
};
use dioxus::prelude::*;

const PRIMARY_PRESETS: [(&str, &str, &str, &str); 3] = [
    ("Blue", "#1677ff", "#4096ff", "#0958d9"),
    ("Cyan", "#13c2c2", "#36cfc9", "#08979c"),
    ("Orange", "#fa8c16", "#ffa940", "#d46b08"),
];

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DemoShell {}
        }
    }
}

#[component]
fn DemoShell() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let preset = use_signal(|| 0usize);
    let ghost = use_signal(|| false);
    let danger = use_signal(|| false);
    let loading = use_signal(|| false);
    let block = use_signal(|| false);

    use_effect(move || {
        let idx = *preset.read();
        let mode_val = *mode.read();
        let (_label, base, hover, active) = PRIMARY_PRESETS[idx];
        let mut next = Theme::for_mode(mode_val);
        next.tokens.color_primary = base.to_string();
        next.tokens.color_primary_hover = hover.to_string();
        next.tokens.color_primary_active = active.to_string();
        next.tokens.color_link = base.to_string();
        next.tokens.color_link_hover = hover.to_string();
        next.tokens.color_link_active = active.to_string();
        if matches!(mode_val, ThemeMode::Dark) {
            next.tokens.color_bg_base = "#0f0f0f".into();
            next.tokens.color_bg_container = "#1b1b1b".into();
        }
        theme.set_theme(next);
    });

    rsx! {
        div {
            class: "demo-shell",
            style: "padding: 16px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",
            div {
                class: "demo-toolbar",
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 16px;",
                span { "主题：" }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Light,
                    "Light"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| *mode.write() = ThemeMode::Dark,
                    "Dark"
                }
                span { style: "margin-left: 12px;", "主色：" }
                {
                    PRIMARY_PRESETS
                        .iter()
                        .enumerate()
                        .map(|(idx, (label, ..))| {
                            let mut preset = preset.clone();
                            rsx!(
                                Button {
                                    r#type: ButtonType::Text,
                                    onclick: move |_| *preset.write() = idx,
                                    class: if idx == *preset.read() { Some("adui-btn-active".into()) } else { None },
                                    "{label}"
                                }
                            )
                        })
                }
                span { style: "margin-left: 12px;", "开关：" }
                {
                    let ghost_val = *ghost.read();
                    let mut ghost_signal = ghost.clone();
                    let ghost_label = format!("Ghost {}", if ghost_val { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let current = *ghost_signal.read();
                            ghost_signal.set(!current);
                        },
                        {ghost_label}
                    })
                }
                {
                    let danger_val = *danger.read();
                    let mut danger_signal = danger.clone();
                    let danger_label = format!("Danger {}", if danger_val { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let current = *danger_signal.read();
                            danger_signal.set(!current);
                        },
                        {danger_label}
                    })
                }
                {
                    let loading_val = *loading.read();
                    let mut loading_signal = loading.clone();
                    let loading_label = format!("Loading {}", if loading_val { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let current = *loading_signal.read();
                            loading_signal.set(!current);
                        },
                        {loading_label}
                    })
                }
                {
                    let block_val = *block.read();
                    let mut block_signal = block.clone();
                    let block_label = format!("Block {}", if block_val { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let current = *block_signal.read();
                            block_signal.set(!current);
                        },
                        {block_label}
                    })
                }
            }

            div {
                class: "demo-section",
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 12px;",
                div {
                    style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container); display: flex; flex-wrap: wrap; gap: 8px;",
                    div { style: "font-weight: 600; margin-bottom: 8px; width: 100%; color: var(--adui-color-text);", "Primary & Default" }
                    Button {
                        r#type: ButtonType::Primary,
                        ghost: *ghost.read(),
                        danger: *danger.read(),
                        loading: *loading.read(),
                        block: *block.read(),
                        "Primary"
                    }
                    Button {
                        r#type: ButtonType::Default,
                        ghost: *ghost.read(),
                        danger: *danger.read(),
                        loading: *loading.read(),
                        block: *block.read(),
                        "Default"
                    }
                    Button {
                        r#type: ButtonType::Dashed,
                        ghost: *ghost.read(),
                        danger: *danger.read(),
                        loading: *loading.read(),
                        block: *block.read(),
                        "Dashed"
                    }
                }

                div {
                    style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container); display: flex; flex-wrap: wrap; gap: 8px;",
                    div { style: "font-weight: 600; margin-bottom: 8px; width: 100%; color: var(--adui-color-text);", "Text & Link" }
                    Button {
                        r#type: ButtonType::Text,
                        danger: *danger.read(),
                        loading: *loading.read(),
                        "Text button"
                    }
                    Button {
                        r#type: ButtonType::Link,
                        danger: *danger.read(),
                        loading: *loading.read(),
                        href: Some("https://ant.design".to_string()),
                        "Link to ant.design"
                    }
                }

                div {
                    style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container); display: flex; flex-wrap: wrap; gap: 8px;",
                    div { style: "font-weight: 600; margin-bottom: 8px; width: 100%; color: var(--adui-color-text);", "Shape & Size" }
                    Button {
                        r#type: ButtonType::Primary,
                        shape: ButtonShape::Round,
                        size: ButtonSize::Small,
                        "Round small"
                    }
                    Button {
                        r#type: ButtonType::Primary,
                        shape: ButtonShape::Default,
                        size: ButtonSize::Middle,
                        "Default middle"
                    }
                    Button {
                        r#type: ButtonType::Primary,
                        shape: ButtonShape::Circle,
                        size: ButtonSize::Large,
                        icon: rsx!(span { "★" }),
                    }
                }
            }
        }
    }
}
