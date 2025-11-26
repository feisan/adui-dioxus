use adui_dioxus::components::space::SpaceSize;
use adui_dioxus::{
    Flex, FlexAlign, FlexComponent, FlexConfigProvider, FlexGap, FlexJustify, FlexSharedConfig,
    FlexWrap, Space, SpaceDirection, ThemeProvider,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            DemoPage {}
        }
    }
}

#[component]
fn DemoPage() -> Element {
    let flex_config = FlexSharedConfig {
        class: Some("demo-flex-shared".into()),
        style: Some("gap: 8px;".into()),
        vertical: Some(false),
    };

    rsx! {
        div {
            style: "padding: 24px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",
            h1 { "Flex / Space 示例" }

            h2 { "Flex：direction + wrap + ConfigProvider" }
            FlexConfigProvider { value: flex_config.clone(),
                Flex {
                    component: FlexComponent::Section,
                    justify: FlexJustify::Between,
                    align: FlexAlign::Center,
                    wrap: FlexWrap::Wrap,
                    gap_size: Some(FlexGap::Large),
                    class: Some("demo-flex-surface".into()),
                    Card { title: "配置 1", body: "继承 ConfigProvider 的 class/style" }
                    Card { title: "配置 2", body: "gap_size = Large" }
                    Card { title: "配置 3", body: "wrap=true 时自动换行" }
                }
            }

            h2 { "Space：size / wrap / split" }
            Space {
                direction: SpaceDirection::Horizontal,
                size: SpaceSize::Large,
                wrap: Some(true),
                split: Some(rsx!(span { class: "demo-split", "·" })),
                Card { title: "Large", body: "size=Large + split dot" }
                Card { title: "Wrap", body: "wrap=true 时自动换行" }
                Card { title: "默认 gap", body: "未传 gap 使用 size 预设" }
            }

            h2 { "Space Compact" }
            Space {
                direction: SpaceDirection::Horizontal,
                compact: true,
                gap: Some(16.0),
                Card { title: "Compact", body: "紧凑模式仅靠自定义 gap" }
                Card { title: "自定义", body: "gap=16px" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CardProps {
    title: &'static str,
    body: &'static str,
}

#[component]
fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            style: "min-width: 180px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container);",
            h3 { style: "margin: 0 0 8px 0;", "{props.title}" }
            p { style: "margin: 0; color: var(--adui-color-text-secondary);", "{props.body}" }
        }
    }
}
