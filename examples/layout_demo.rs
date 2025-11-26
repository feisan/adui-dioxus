use adui_dioxus::{
    Button, ButtonType, Col, Content, Divider, DividerOrientation, Flex, FlexAlign, FlexDirection,
    FlexJustify, Footer, Header, Layout, Masonry, MasonryResponsive, Row, RowAlign, RowJustify,
    Sider, SiderTheme, Space, SpaceDirection, Splitter, SplitterOrientation, Text, TextType,
    ThemeMode, ThemeProvider, Title, TitleLevel,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            LayoutDemo {}
        }
    }
}

#[component]
fn LayoutDemo() -> Element {
    let mut mode = use_signal(|| ThemeMode::Light);
    let split_ratio = use_signal(|| 0.4f32);
    let sider_collapsed = use_signal(|| false);
    let mini_collapsed = use_signal(|| true);
    let action_buttons = [
        (ButtonType::Primary, "Action"),
        (ButtonType::Default, "Cancel"),
        (ButtonType::Text, "More"),
    ];

    use_effect(move || {
        adui_dioxus::use_theme().set_mode(*mode.read());
    });

    rsx! {
        div {
            style: "padding: 16px; background: var(--adui-color-bg-base); min-height: 100vh; color: var(--adui-color-text);",
            Title { level: TitleLevel::H3, "Layout Components" }
            div {
                style: "display: flex; gap: 8px; align-items: center; margin-bottom: 16px;",
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Light, "Light" }
                Button { r#type: ButtonType::Default, onclick: move |_| *mode.write() = ThemeMode::Dark, "Dark" }
            }

            {section_card("Divider", rsx! {
                Divider { orientation: DividerOrientation::Left, content: Some(rsx!("Left")) }
                Divider { orientation: DividerOrientation::Center, content: Some(rsx!("Center")) }
                Divider { orientation: DividerOrientation::Right, dashed: true, orientation_margin: Some("24px".into()), content: Some(rsx!("Right Dashed")) }
                Divider { vertical: true }
                Text { r#type: TextType::Secondary, "Inline vertical divider above." }
            })}

            {section_card("Flex & Space", rsx! {
                Flex {
                    direction: FlexDirection::Row,
                    justify: FlexJustify::Between,
                    align: FlexAlign::Center,
                    gap: Some(8.0),
                    {sample_box("A")}
                    {sample_box("B")}
                    {sample_box("C")}
                }
                Space {
                    direction: SpaceDirection::Horizontal,
                    gap: Some(12.0),
                    split: Some(rsx!(Divider { vertical: true })),
                    {action_buttons.into_iter().map(|(kind, label)| rsx!(
                        Button { r#type: kind, "{label}" }
                    ))}
                }
            })}

            {section_card("Grid 24 Columns", rsx! {
                Row {
                    gutter: Some(12.0),
                    justify: RowJustify::Start,
                    align: RowAlign::Top,
                    Col { span: 6, {sample_box("6") } }
                    Col { span: 6, {sample_box("6") } }
                    Col { span: 6, {sample_box("6") } }
                    Col { span: 6, {sample_box("6") } }
                }
                Row {
                    gutter: Some(12.0),
                    Col { span: 8, {sample_box("8")} }
                    Col { span: 8, {sample_box("8")} }
                    Col { span: 8, {sample_box("8")} }
                }
            })}

            {section_card("Layout", rsx! {
                Text { r#type: TextType::Secondary, "Sider 支持 collapsible + 受控 collapsed" }
                Button {
                    r#type: ButtonType::Default,
                    onclick: {
                        let mut sig = sider_collapsed;
                        move |_| {
                            let next = {
                                let current = *sig.read();
                                !current
                            };
                            sig.set(next);
                        }
                    },
                    if *sider_collapsed.read() { "展开" } else { "收起" }
                }
                Layout {
                    has_sider: Some(true),
                    Sider {
                        collapsible: true,
                        theme: SiderTheme::Dark,
                        width: Some(220.0),
                        collapsed_width: Some(72.0),
                        collapsed: Some(*sider_collapsed.read()),
                        on_collapse: {
                            let mut sig = sider_collapsed;
                            move |next| sig.set(next)
                        },
                        {sample_bar("导航菜单")}
                    }
                    Layout {
                        Header { {sample_bar("Header")} }
                        Content { {sample_bar("Content")} }
                        Footer { {sample_bar("Footer")} }
                    }
                }
                Divider { }
                Text { r#type: TextType::Secondary, "Zero Width Trigger（collapsed_width = 0）" }
                Layout {
                    has_sider: Some(true),
                    Sider {
                        theme: SiderTheme::Light,
                        collapsible: true,
                        collapsed_width: Some(0.0),
                        width: Some(180.0),
                        zero_width_trigger_style: Some("top: 24px;".into()),
                        collapsed: Some(*mini_collapsed.read()),
                        on_collapse: {
                            let mut sig = mini_collapsed;
                            move |next| sig.set(next)
                        },
                        {sample_bar("Mini Sider")}
                    }
                    Content {
                        {sample_bar("Content Area")}
                    }
                }
            })}

            {section_card("Masonry", rsx! {
                Masonry {
                    columns: 4,
                    responsive: Some(MasonryResponsive {
                        xs: Some(1),
                        sm: Some(2),
                        md: Some(3),
                        lg: Some(4),
                        ..Default::default()
                    }),
                    gap: Some(12.0),
                    row_gap: Some(20.0),
                    min_column_width: Some(180.0),
                    {
                        (0..6).map(|i| {
                            let h = 60 + i * 20;
                            rsx!(
                                div {
                                    style: format!("background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 8px; height:{}px;", h),
                                    "Card {i}"
                                }
                            )
                        })
                    }
                }
            })}

            {section_card("Splitter", rsx! {
                {
                    let value = *split_ratio.read();
                    let label = format!("{:.0}% / {:.0}%", value * 100.0, (1.0 - value) * 100.0);
                    rsx!(Text { r#type: TextType::Secondary, {label} })
                }
                Text { r#type: TextType::Secondary, "拖动中线可调整 Pane 宽度" }
                Splitter {
                    orientation: SplitterOrientation::Horizontal,
                    split: Some(*split_ratio.read()),
                    on_change: {
                        let mut ratio_sig = split_ratio;
                        move |v| {
                            ratio_sig.set(v);
                            println!("split changed to {:.2}", v);
                        }
                    },
                    first: rsx!({sample_bar("Pane A")}),
                    second: rsx!({sample_bar("Pane B")}),
                }
            })}
        }
    }
}

fn sample_box(label: &str) -> Element {
    rsx! {
        div {
            style: "background: var(--adui-color-bg-container); border: 1px solid var(--adui-color-border); padding: 12px; text-align: center; border-radius: var(--adui-radius);",
            "{label}"
        }
    }
}

fn sample_bar(label: &str) -> Element {
    rsx! {
        div { style: "padding: 12px; border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); background: var(--adui-color-bg-container);", "{label}" }
    }
}

fn section_card(title: &str, body: Element) -> Element {
    rsx! {
        div {
            style: "border: 1px solid var(--adui-color-border); border-radius: var(--adui-radius); padding: 12px; background: var(--adui-color-bg-container); display: flex; flex-direction: column; gap: 12px; margin-bottom: 16px;",
            span { style: "font-weight: 600; color: var(--adui-color-text);", "{title}" }
            {body}
        }
    }
}
