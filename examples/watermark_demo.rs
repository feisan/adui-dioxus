//! Watermark component demo.

use adui_dioxus::{
    App, Button, ButtonType, Card, ComponentSize, ConfigProvider, Watermark, WatermarkFont,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { WatermarkDemoShell {} }
        }
    }
}

/// Helper component to conditionally wrap content with Watermark
#[component]
fn ConditionalWatermark(
    show: bool,
    content: Option<Vec<String>>,
    image: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
    font: Option<WatermarkFont>,
    rotate: Option<f32>,
    gap: Option<[f32; 2]>,
    children: Element,
) -> Element {
    if show {
        rsx! {
            Watermark {
                content: content,
                image: image,
                width: width,
                height: height,
                font: font,
                rotate: rotate.unwrap_or(-22.0),
                gap: gap,
                {children}
            }
        }
    } else {
        rsx! { {children} }
    }
}

#[component]
fn WatermarkDemoShell() -> Element {
    let show_watermark = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Watermark demo" }
            p { "展示 Watermark 组件的各种用法，包括文字水印、图片水印和自定义样式。" }

            // 控制按钮
            div { style: "margin-bottom: 16px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut show_watermark = show_watermark;
                        move |_| {
                            let current = *show_watermark.read();
                            show_watermark.set(!current);
                        }
                    },
                    if *show_watermark.read() { "隐藏所有水印" } else { "显示所有水印" }
                }
            }

            // 基础文字水印
            h3 { style: "margin-top: 24px;", "基础文字水印" }
            ConditionalWatermark {
                show: *show_watermark.read(),
                content: Some(vec!["Ant Design".to_string()]),
                Card {
                    title: Some(rsx!("卡片标题")),
                    style: Some("height: 200px;".to_string()),
                    children: rsx! {
                        p { "这是一段受水印保护的内容。" }
                        p { "水印会覆盖整个容器区域。" }
                    }
                }
            }

            // 多行文字水印
            h3 { style: "margin-top: 24px;", "多行文字水印" }
            ConditionalWatermark {
                show: *show_watermark.read(),
                content: Some(vec![
                    "Confidential".to_string(),
                    "2024-01-01".to_string(),
                ]),
                Card {
                    style: Some("height: 200px;".to_string()),
                    children: rsx! {
                        p { "多行水印可以显示更多信息，例如日期、用户名等。" }
                    }
                }
            }

            // 自定义字体样式
            h3 { style: "margin-top: 24px;", "自定义字体样式" }
            ConditionalWatermark {
                show: *show_watermark.read(),
                content: Some(vec!["Custom Style".to_string()]),
                font: Some(WatermarkFont {
                    color: "rgba(255, 0, 0, 0.15)".to_string(),
                    font_size: 20.0,
                    font_weight: "bold".to_string(),
                    ..Default::default()
                }),
                rotate: Some(-30.0),
                Card {
                    style: Some("height: 200px;".to_string()),
                    children: rsx! {
                        p { "自定义红色水印，更大的字体和不同的旋转角度。" }
                    }
                }
            }

            // 调整间距
            h3 { style: "margin-top: 24px;", "调整水印间距" }
            ConditionalWatermark {
                show: *show_watermark.read(),
                content: Some(vec!["Dense".to_string()]),
                gap: Some([50.0, 50.0]),
                Card {
                    style: Some("height: 200px;".to_string()),
                    children: rsx! {
                        p { "更密集的水印分布，间距设置为 50x50 像素。" }
                    }
                }
            }

            // 图片水印
            h3 { style: "margin-top: 24px;", "图片水印" }
            ConditionalWatermark {
                show: *show_watermark.read(),
                image: Some("https://gw.alipayobjects.com/zos/bmw-prod/59a18171-ae17-4571-bbbd-07a66b520b46/k7cjl1fa_w192_h106.png".to_string()),
                width: Some(100.0),
                height: Some(55.0),
                Card {
                    style: Some("height: 200px;".to_string()),
                    children: rsx! {
                        p { "使用图片作为水印，适合添加公司 Logo 等。" }
                    }
                }
            }

            // 全页水印示例说明
            h3 { style: "margin-top: 24px;", "全页水印" }
            p { style: "color: var(--adui-color-text-secondary);",
                "将 Watermark 包裹整个页面内容即可实现全页水印效果。"
            }
        }
    }
}
