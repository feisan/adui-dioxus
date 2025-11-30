//! Typography 组件演示
//!
//! 展示 Typography 组件的基础用法和高级用法，包括：
//! - Title 标题组件
//! - Text 文本组件
//! - Paragraph 段落组件
//! - 文本样式（strong、italic、underline等）
//! - 可编辑和可复制功能
//! - 省略号功能

use adui_dioxus::{
    Button, ButtonType, Paragraph, Text, TextType, ThemeMode, ThemeProvider, Title, TitleLevel,
    TypographyCopyable, TypographyEditable, TypographyEllipsis,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            TypographyDemo {}
        }
    }
}

#[component]
fn TypographyDemo() -> Element {
    let mut mode = use_signal(|| ThemeMode::Light);
    let mut paragraph_text =
        use_signal(|| "这是一个可编辑段落，点击右侧图标即可切换为输入状态。".to_string());

    use_effect(move || {
        adui_dioxus::use_theme().set_mode(*mode.read());
    });

    let sample_long = "Ant Design Dioxus Typography 示例：可组合 strong/italic/underline/delete/code/mark，支持 ellipsis 和 tone。这是一个较长的文本示例，用于演示省略号功能。";

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",

            // 控制工具栏
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px; align-items: center; margin-bottom: 24px; padding: 12px; background: var(--adui-color-bg-container); border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border);",
                span { style: "font-weight: 600;", "主题控制：" }
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
            }

            Title { level: TitleLevel::H2, style: "margin-bottom: 16px;", "基础用法" }

            // Title 标题
            DemoSection {
                title: "Title 标题",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Title { level: TitleLevel::H1, "H1 标题" }
                    Title { level: TitleLevel::H2, "H2 标题" }
                    Title { level: TitleLevel::H3, "H3 标题" }
                    Title { level: TitleLevel::H4, "H4 标题" }
                    Title { level: TitleLevel::H5, "H5 标题" }
                }
            }

            // Text 文本
            DemoSection {
                title: "Text 文本",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    Text { "默认文本" }
                    Text { r#type: TextType::Secondary, "次要文本" }
                    Text { r#type: TextType::Success, "成功文本" }
                    Text { r#type: TextType::Warning, "警告文本" }
                    Text { r#type: TextType::Danger, "危险文本" }
                    Text { r#type: TextType::Disabled, "禁用文本" }
                }
            }

            // Paragraph 段落
            DemoSection {
                title: "Paragraph 段落",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Paragraph {
                        "这是一个段落组件，用于显示较长的文本内容。它支持多行显示，适合用于正文和描述性文本。"
                    }
                    Paragraph {
                        r#type: TextType::Secondary,
                        "次要颜色的段落，通常用于辅助说明或次要信息。"
                    }
                    Paragraph {
                        r#type: TextType::Danger,
                        underline: true,
                        "危险提示段落，可与 underline 等样式组合使用。"
                    }
                }
            }

            Title { level: TitleLevel::H2, style: "margin: 32px 0 16px 0;", "高级用法" }

            // 文本样式
            DemoSection {
                title: "文本样式",
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    Text { strong: true, "粗体文本 (strong)" }
                    Text { italic: true, "斜体文本 (italic)" }
                    Text { underline: true, "下划线文本 (underline)" }
                    Text { delete: true, "删除线文本 (delete)" }
                    Text { mark: true, "标记文本 (mark)" }
                    Text { code: true, "代码文本 (code)" }
                    Text {
                        strong: true,
                        italic: true,
                        underline: true,
                        "组合样式：粗体 + 斜体 + 下划线"
                    }
                }
            }

            // 可复制文本
            DemoSection {
                title: "可复制文本",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Text {
                        copyable: Some(TypographyCopyable {
                            text: "这是可复制的文本内容".to_string(),
                            tooltips: Some(("复制".into(), "已复制".into())),
                            icon: None,
                            copied_icon: None,
                        }),
                        "点击图标复制文本"
                    }
                    Text {
                        r#type: TextType::Secondary,
                        copyable: Some(TypographyCopyable::new(sample_long)),
                        "{sample_long}"
                    }
                }
            }

            // 可编辑文本
            DemoSection {
                title: "可编辑文本",
                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",
                    Paragraph {
                        editable: Some(TypographyEditable {
                            text: Some(paragraph_text.read().clone()),
                            placeholder: Some("请输入段落内容".into()),
                            auto_focus: true,
                            ..Default::default()
                        }),
                        on_edit: move |value: String| paragraph_text.set(value),
                        on_edit_cancel: move |_| {},
                        "{paragraph_text.read().clone()}"
                    }
                    Text {
                        r#type: TextType::Secondary,
                        style: Some("font-size: 12px;".into()),
                        "点击右侧编辑图标可以编辑文本内容"
                    }
                }
            }

            // 省略号
            DemoSection {
                title: "省略号功能",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600; font-size: 14px;", "单行省略：" }
                        Text {
                            ellipsis: true,
                            style: Some("max-width: 300px; display: inline-block;".into()),
                            "{sample_long}"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600; font-size: 14px;", "多行省略（2行）：" }
                        Text {
                            r#type: TextType::Secondary,
                            ellipsis: true,
                            ellipsis_config: Some(TypographyEllipsis {
                                rows: Some(2),
                                expandable: true,
                                tooltip: Some("点击展开".into()),
                                expand_text: None,
                                collapse_text: None,
                            }),
                            style: Some("max-width: 400px; display: inline-block;".into()),
                            "{sample_long}"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-weight: 600; font-size: 14px;", "多行省略（3行，可展开）：" }
                        Text {
                            r#type: TextType::Secondary,
                            ellipsis: true,
                            ellipsis_config: Some(TypographyEllipsis {
                                rows: Some(3),
                                expandable: true,
                                tooltip: Some("点击展开查看完整内容".into()),
                                expand_text: None,
                                collapse_text: None,
                            }),
                            style: Some("max-width: 400px; display: inline-block;".into()),
                            "{sample_long} {sample_long}"
                        }
                    }
                }
            }

            // 组合示例
            DemoSection {
                title: "组合示例",
                div {
                    style: "display: flex; flex-direction: column; gap: 16px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        Title {
                            level: TitleLevel::H3,
                            "文章标题"
                        }
                        Text {
                            r#type: TextType::Secondary,
                            style: Some("font-size: 14px;".into()),
                            "2024-01-01 作者：张三"
                        }
                        Paragraph {
                            r#type: TextType::Default,
                            "这是文章的正文内容。Typography 组件提供了丰富的文本样式和功能，可以满足各种文本展示需求。"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        Text {
                            strong: true,
                            mark: true,
                            "重要提示："
                        }
                        Text {
                            r#type: TextType::Secondary,
                            "这是一条重要信息，使用了 strong 和 mark 样式来突出显示。"
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        Text {
                            code: true,
                            copyable: Some(TypographyCopyable::new("npm install adui-dioxus")),
                            "npm install adui-dioxus"
                        }
                        Text {
                            r#type: TextType::Secondary,
                            style: Some("font-size: 12px;".into()),
                            "点击复制命令"
                        }
                    }
                }
            }
        }
    }
}

// 统一的demo section组件
#[derive(Props, Clone, PartialEq)]
struct DemoSectionProps {
    title: &'static str,
    children: Element,
}

#[component]
fn DemoSection(props: DemoSectionProps) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 24px; padding: 16px; background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius);",
            div {
                style: "font-weight: 600; margin-bottom: 12px; color: var(--adui-color-text); font-size: 14px;",
                {props.title}
            }
            {props.children}
        }
    }
}
