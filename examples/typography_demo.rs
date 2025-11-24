use adui_dioxus::{
    Button, ButtonType, Paragraph, Text, TextType, ThemeMode, ThemeProvider, Title, TitleLevel,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TypographyPlayground {}
        }
    }
}

#[component]
fn TypographyPlayground() -> Element {
    let mut mode = use_signal(|| ThemeMode::Light);
    let strong = use_signal(|| false);
    let italic = use_signal(|| false);
    let underline = use_signal(|| false);
    let delete = use_signal(|| false);
    let mark = use_signal(|| false);
    let code = use_signal(|| false);
    let ellipsis = use_signal(|| true);
    let mut tone = use_signal(|| TextType::Default);

    use_effect(move || {
        // propagate theme mode if toggled
        adui_dioxus::use_theme().set_mode(*mode.read());
    });

    let sample_long = "Ant Design Dioxus Typography 示例：可组合 strong/italic/underline/delete/code/mark，支持 ellipsis 和 tone。";

    rsx! {
        div {
            style: "padding: 16px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 12px;",
                span { "主题：" }
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Light, "Light" }
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Dark, "Dark" }
                span { style: "margin-left: 12px;", "Tone：" }
                Button { r#type: ButtonType::Text, onclick: move |_| *tone.write() = TextType::Default, "Default" }
                Button { r#type: ButtonType::Text, onclick: move |_| *tone.write() = TextType::Secondary, "Secondary" }
                Button { r#type: ButtonType::Text, onclick: move |_| *tone.write() = TextType::Danger, "Danger" }
                Button { r#type: ButtonType::Text, onclick: move |_| *tone.write() = TextType::Disabled, "Disabled" }
                span { style: "margin-left: 12px;", "修饰：" }
                {
                    let mut sig = strong;
                    let label = format!("Strong {}", if *strong.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = italic;
                    let label = format!("Italic {}", if *italic.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = underline;
                    let label = format!("Underline {}", if *underline.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = delete;
                    let label = format!("Delete {}", if *delete.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = mark;
                    let label = format!("Mark {}", if *mark.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = code;
                    let label = format!("Code {}", if *code.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
                {
                    let mut sig = ellipsis;
                    let label = format!("Ellipsis {}", if *ellipsis.read() { "ON" } else { "OFF" });
                    rsx!(Button { r#type: ButtonType::Text, onclick: move |_| { let cur = *sig.read(); sig.set(!cur); }, {label} })
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 12px;",
                {demo_card("Title Levels", rsx! {
                    Title { level: TitleLevel::H1, r#type: *tone.read(), "Typography H1" }
                    Title { level: TitleLevel::H2, r#type: *tone.read(), "Typography H2" }
                    Title { level: TitleLevel::H3, r#type: *tone.read(), "Typography H3" }
                    Title { level: TitleLevel::H4, r#type: *tone.read(), "Typography H4" }
                    Title { level: TitleLevel::H5, r#type: *tone.read(), "Typography H5" }
                })}

                {demo_card("Text Variants", rsx! {
                    Text {
                        r#type: *tone.read(),
                        strong: *strong.read(),
                        italic: *italic.read(),
                        underline: *underline.read(),
                        delete: *delete.read(),
                        mark: *mark.read(),
                        code: *code.read(),
                        ellipsis: *ellipsis.read(),
                        wrap: false,
                        style: Some("max-width: 240px; display: inline-block;".into()),
                        "{sample_long}"
                    }
                })}

                {demo_card("Paragraph", rsx! {
                    Paragraph {
                        r#type: TextType::Secondary,
                        "Paragraph 默认行高，支持跨行文字，适合正文与描述。"
                    }
                    Paragraph {
                        r#type: TextType::Danger,
                        underline: true,
                        "警示段落，可与 danger/underline 组合。"
                    }
                })}
            }
        }
    }
}

fn demo_card(title: &str, body: Element) -> Element {
    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 8px;",
            span { style: "font-weight: 600; color: var(--adui-color-text);", "{title}" }
            {body}
        }
    }
}
