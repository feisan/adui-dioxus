use crate::components::icon::{Icon, IconKind};
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Timeline mode (position of items).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineMode {
    /// All items on the left (or top in horizontal).
    #[default]
    Left,
    /// All items on the right (or bottom in horizontal).
    Right,
    /// Items alternate between left and right.
    Alternate,
}

impl TimelineMode {
    fn as_class(&self) -> &'static str {
        match self {
            TimelineMode::Left => "adui-timeline-left",
            TimelineMode::Right => "adui-timeline-right",
            TimelineMode::Alternate => "adui-timeline-alternate",
        }
    }
}

/// Timeline orientation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl TimelineOrientation {
    fn as_class(&self) -> &'static str {
        match self {
            TimelineOrientation::Vertical => "adui-timeline-vertical",
            TimelineOrientation::Horizontal => "adui-timeline-horizontal",
        }
    }
}

/// Color presets for timeline items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineColor {
    Blue,
    Green,
    Red,
    Gray,
}

impl TimelineColor {
    fn as_class(&self) -> &'static str {
        match self {
            TimelineColor::Blue => "adui-timeline-item-blue",
            TimelineColor::Green => "adui-timeline-item-green",
            TimelineColor::Red => "adui-timeline-item-red",
            TimelineColor::Gray => "adui-timeline-item-gray",
        }
    }
}

/// Data model for a single timeline item.
#[derive(Clone, PartialEq)]
pub struct TimelineItem {
    pub key: String,
    pub title: Option<Element>,
    pub content: Option<Element>,
    pub icon: Option<Element>,
    pub color: Option<TimelineColor>,
    /// Whether this item is in pending/loading state.
    pub pending: bool,
}

impl TimelineItem {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: None,
            content: None,
            icon: None,
            color: None,
            pending: false,
        }
    }

    pub fn title(mut self, title: Element) -> Self {
        self.title = Some(title);
        self
    }

    pub fn content(mut self, content: Element) -> Self {
        self.content = Some(content);
        self
    }

    pub fn icon(mut self, icon: Element) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn color(mut self, color: TimelineColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }
}

/// Props for the Timeline component.
#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    /// Timeline items to display.
    pub items: Vec<TimelineItem>,
    /// Timeline mode (position).
    #[props(default)]
    pub mode: TimelineMode,
    /// Timeline orientation.
    #[props(default)]
    pub orientation: TimelineOrientation,
    /// Whether to reverse the order of items.
    #[props(default)]
    pub reverse: bool,
    /// Extra class name.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Timeline component.
#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let TimelineProps {
        items,
        mode,
        orientation,
        reverse,
        class,
        style,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();

    // Build root classes
    let mut class_list = vec!["adui-timeline".to_string()];
    class_list.push(mode.as_class().to_string());
    class_list.push(orientation.as_class().to_string());
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let style_attr = style.unwrap_or_default();

    // Prepare items in the correct order
    let display_items: Vec<&TimelineItem> = if reverse {
        items.iter().rev().collect()
    } else {
        items.iter().collect()
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "list",
            {display_items.iter().enumerate().map(|(idx, item)| {
                let key = item.key.clone();
                let title = item.title.clone();
                let content = item.content.clone();
                let icon = item.icon.clone();
                let color = item.color;
                let pending = item.pending;

                // Determine position for alternate mode
                let is_left = match mode {
                    TimelineMode::Left => true,
                    TimelineMode::Right => false,
                    TimelineMode::Alternate => idx % 2 == 0,
                };

                let mut item_class = vec!["adui-timeline-item".to_string()];
                if pending {
                    item_class.push("adui-timeline-item-pending".into());
                }
                if is_left {
                    item_class.push("adui-timeline-item-left".into());
                } else {
                    item_class.push("adui-timeline-item-right".into());
                }
                if let Some(c) = color {
                    item_class.push(c.as_class().into());
                }
                let item_class_attr = item_class.join(" ");

                rsx! {
                    div {
                        key: "{key}",
                        class: "{item_class_attr}",
                        role: "listitem",
                        div { class: "adui-timeline-item-tail",
                            style: "border-left-color: {tokens.color_border};"
                        },
                        div { class: "adui-timeline-item-head",
                            style: "border-color: {tokens.color_border};",
                            {if let Some(icon_elem) = icon {
                                Some(rsx! {
                                    span { class: "adui-timeline-item-icon",
                                        {icon_elem}
                                    }
                                })
                            } else if pending {
                                Some(rsx! {
                                    span { class: "adui-timeline-item-icon",
                                        Icon {
                                            kind: IconKind::Loading,
                                            spin: true,
                                        }
                                    }
                                })
                            } else {
                                None
                            }}
                        },
                        div { class: "adui-timeline-item-content",
                            {title.map(|t| rsx! {
                                div { class: "adui-timeline-item-title",
                                    {t}
                                }
                            })},
                            {content.map(|c| rsx! {
                                div { class: "adui-timeline-item-description",
                                    {c}
                                }
                            })},
                        }
                    }
                }
            })}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_mode_class_mapping_is_stable() {
        assert_eq!(TimelineMode::Left.as_class(), "adui-timeline-left");
        assert_eq!(TimelineMode::Right.as_class(), "adui-timeline-right");
        assert_eq!(
            TimelineMode::Alternate.as_class(),
            "adui-timeline-alternate"
        );
    }

    #[test]
    fn timeline_orientation_class_mapping_is_stable() {
        assert_eq!(
            TimelineOrientation::Vertical.as_class(),
            "adui-timeline-vertical"
        );
        assert_eq!(
            TimelineOrientation::Horizontal.as_class(),
            "adui-timeline-horizontal"
        );
    }

    #[test]
    fn timeline_color_class_mapping_is_stable() {
        assert_eq!(TimelineColor::Blue.as_class(), "adui-timeline-item-blue");
        assert_eq!(TimelineColor::Green.as_class(), "adui-timeline-item-green");
        assert_eq!(TimelineColor::Red.as_class(), "adui-timeline-item-red");
        assert_eq!(TimelineColor::Gray.as_class(), "adui-timeline-item-gray");
    }
}
