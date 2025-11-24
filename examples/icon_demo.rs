use adui_dioxus::{
    Button, ButtonType, Icon, IconKind, ThemeMode, ThemeProvider, ThemeTokens, use_theme,
};
use dioxus::prelude::*;

const ICON_LIST: &[(IconKind, &str)] = &[
    (IconKind::Plus, "Plus"),
    (IconKind::Minus, "Minus"),
    (IconKind::Check, "Check"),
    (IconKind::Close, "Close"),
    (IconKind::Info, "Info"),
    (IconKind::Question, "Question"),
    (IconKind::Search, "Search"),
    (IconKind::ArrowLeft, "ArrowLeft"),
    (IconKind::ArrowRight, "ArrowRight"),
    (IconKind::Loading, "Loading"),
];

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            IconPlayground {}
        }
    }
}

#[component]
fn IconPlayground() -> Element {
    let theme = use_theme();
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut size = use_signal(|| 24f32);
    let mut accent = use_signal(|| ThemeTokens::light().color_primary);
    let spin_all = use_signal(|| false);

    let size_val_display = *size.read();
    let spin_flag = *spin_all.read();
    let accent_val = accent.read().clone();

    use_effect(move || {
        let m = *mode.read();
        theme.set_mode(m);
        if matches!(m, ThemeMode::Dark) {
            let mut tokens = ThemeTokens::dark();
            tokens.color_primary = accent.read().clone();
            theme.set_theme(adui_dioxus::Theme { mode: m, tokens });
        } else {
            let mut tokens = ThemeTokens::light();
            tokens.color_primary = accent.read().clone();
            theme.set_theme(adui_dioxus::Theme { mode: m, tokens });
        }
    });

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base); color: var(--adui-color-text);",
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 12px;",
                span { "主题：" }
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Light, "Light" }
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Dark, "Dark" }
                span { style: "margin-left: 12px;", "大小：" }
                Button { r#type: ButtonType::Text, onclick: move |_| { let mut v = *size.read(); v = (v - 2.0).max(12.0); size.set(v); }, "−" }
                span { "{size_val_display:.0}px" }
                Button { r#type: ButtonType::Text, onclick: move |_| { let mut v = *size.read(); v = (v + 2.0).min(48.0); size.set(v); }, "+" }
                span { style: "margin-left: 12px;", "主色：" }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#1677ff".into()),
                    "蓝"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#fa8c16".into()),
                    "橙"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#13c2c2".into()),
                    "青"
                }
                Button {
                    r#type: ButtonType::Text,
                    onclick: move |_| accent.set("#ff4d4f".into()),
                    "红"
                }
                {
                    let mut spin_sig = spin_all;
                    let spin_label = format!("全局旋转 {}", if spin_flag { "ON" } else { "OFF" });
                    rsx!(Button {
                        r#type: ButtonType::Text,
                        onclick: move |_| {
                            let cur = *spin_sig.read();
                            spin_sig.set(!cur);
                        },
                        {spin_label}
                    })
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 12px;",
                {
                    ICON_LIST.iter().map(|(kind, label)| {
                        let color = accent_val.clone();
                        let spinning = spin_flag || matches!(kind, IconKind::Loading);
                        let size_val = size_val_display;
                        rsx! {
                            div {
                                style: "display: flex; align-items: center; gap: 10px; padding: 10px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); background: var(--adui-color-bg-container);",
                                Icon {
                                    kind: *kind,
                                    size: size_val,
                                    color: Some(color.clone()),
                                    spin: spinning,
                                    aria_label: Some(label.to_string()),
                                }
                                span { style: "color: var(--adui-color-text);", "{label}" }
                            }
                        }
                    })
                },
            }
        }
    }
}
