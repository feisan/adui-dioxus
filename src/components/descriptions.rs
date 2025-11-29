use crate::components::config_provider::{ComponentSize, use_config};
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Layout direction for descriptions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptionsLayout {
    #[default]
    Horizontal,
    Vertical,
}

impl DescriptionsLayout {
    fn as_class(&self) -> &'static str {
        match self {
            DescriptionsLayout::Horizontal => "adui-descriptions-horizontal",
            DescriptionsLayout::Vertical => "adui-descriptions-vertical",
        }
    }
}

/// Size variants for Descriptions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DescriptionsSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl DescriptionsSize {
    fn from_global(size: ComponentSize) -> Self {
        match size {
            ComponentSize::Small => DescriptionsSize::Small,
            ComponentSize::Large => DescriptionsSize::Large,
            ComponentSize::Middle => DescriptionsSize::Middle,
        }
    }

    fn as_class(&self) -> &'static str {
        match self {
            DescriptionsSize::Small => "adui-descriptions-sm",
            DescriptionsSize::Middle => "adui-descriptions-md",
            DescriptionsSize::Large => "adui-descriptions-lg",
        }
    }
}

/// Responsive column configuration for different screen sizes.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponsiveColumn {
    /// Default column count.
    pub default: usize,
    /// Column count for xs screens (< 576px).
    pub xs: Option<usize>,
    /// Column count for sm screens (≥ 576px).
    pub sm: Option<usize>,
    /// Column count for md screens (≥ 768px).
    pub md: Option<usize>,
    /// Column count for lg screens (≥ 992px).
    pub lg: Option<usize>,
    /// Column count for xl screens (≥ 1200px).
    pub xl: Option<usize>,
    /// Column count for xxl screens (≥ 1600px).
    pub xxl: Option<usize>,
}

impl ResponsiveColumn {
    pub fn new(default: usize) -> Self {
        Self {
            default,
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }
}

/// Column configuration (simple or responsive).
#[derive(Clone, Debug, PartialEq)]
pub enum ColumnConfig {
    Simple(usize),
    Responsive(ResponsiveColumn),
}

impl Default for ColumnConfig {
    fn default() -> Self {
        ColumnConfig::Simple(3)
    }
}

impl ColumnConfig {
    /// Get the effective column count (for now, just return default/simple value).
    /// In a full implementation, this would check window width.
    fn get_columns(&self) -> usize {
        match self {
            ColumnConfig::Simple(n) => *n,
            ColumnConfig::Responsive(r) => r.default,
        }
    }
}

/// Data model for a single description item.
#[derive(Clone, PartialEq)]
pub struct DescriptionsItem {
    pub key: String,
    pub label: Element,
    pub content: Element,
    /// How many columns this item spans.
    pub span: usize,
}

impl DescriptionsItem {
    pub fn new(key: impl Into<String>, label: Element, content: Element) -> Self {
        Self {
            key: key.into(),
            label,
            content,
            span: 1,
        }
    }

    pub fn span(mut self, span: usize) -> Self {
        self.span = span.max(1);
        self
    }
}

/// Props for the Descriptions component.
#[derive(Props, Clone, PartialEq)]
pub struct DescriptionsProps {
    /// Description items to display.
    pub items: Vec<DescriptionsItem>,
    /// Optional title for the descriptions.
    #[props(optional)]
    pub title: Option<Element>,
    /// Optional extra content in the header.
    #[props(optional)]
    pub extra: Option<Element>,
    /// Whether to show border.
    #[props(default)]
    pub bordered: bool,
    /// Layout direction.
    #[props(default)]
    pub layout: DescriptionsLayout,
    /// Column configuration.
    #[props(default)]
    pub column: ColumnConfig,
    /// Size variant.
    #[props(optional)]
    pub size: Option<DescriptionsSize>,
    /// Whether to show colon after label.
    #[props(default = true)]
    pub colon: bool,
    /// Extra class name.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Descriptions component.
#[component]
pub fn Descriptions(props: DescriptionsProps) -> Element {
    let DescriptionsProps {
        items,
        title,
        extra,
        bordered,
        layout,
        column,
        size,
        colon,
        class,
        style,
    } = props;

    let config = use_config();
    let theme = use_theme();
    let tokens = theme.tokens();

    // Resolve size
    let resolved_size = if let Some(s) = size {
        s
    } else {
        DescriptionsSize::from_global(config.size)
    };

    let columns = column.get_columns();

    // Build root classes
    let mut class_list = vec!["adui-descriptions".to_string()];
    class_list.push(resolved_size.as_class().to_string());
    class_list.push(layout.as_class().to_string());
    if bordered {
        class_list.push("adui-descriptions-bordered".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let style_attr = format!(
        "border-color:{};{}",
        tokens.color_border,
        style.unwrap_or_default()
    );

    // Split items into rows based on column count and span
    let mut rows: Vec<Vec<&DescriptionsItem>> = Vec::new();
    let mut current_row: Vec<&DescriptionsItem> = Vec::new();
    let mut current_span = 0;

    for item in &items {
        let item_span = item.span.min(columns);
        if current_span + item_span > columns && !current_row.is_empty() {
            rows.push(current_row);
            current_row = Vec::new();
            current_span = 0;
        }
        current_row.push(item);
        current_span += item_span;
        if current_span >= columns {
            rows.push(current_row);
            current_row = Vec::new();
            current_span = 0;
        }
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            {(title.is_some() || extra.is_some()).then(|| rsx! {
                div { class: "adui-descriptions-header",
                    {title.map(|t| rsx! {
                        div { class: "adui-descriptions-title",
                            {t}
                        }
                    })},
                    {extra.map(|e| rsx! {
                        div { class: "adui-descriptions-extra",
                            {e}
                        }
                    })},
                }
            })},
            div { class: "adui-descriptions-view",
                {if bordered {
                    rsx! {
                        table { class: "adui-descriptions-table",
                            tbody {
                                {rows.iter().map(|row| {
                                    if layout == DescriptionsLayout::Horizontal {
                                        rsx! {
                                            tr { class: "adui-descriptions-row",
                                                {row.iter().map(|item| {
                                                    let label = item.label.clone();
                                                    let content = item.content.clone();
                                                    let span = item.span;
                                                    rsx! {
                                                        th {
                                                            class: "adui-descriptions-item-label",
                                                            colspan: "{span}",
                                                            {label}
                                                            {colon.then(|| rsx! { span { class: "adui-descriptions-colon", ":" } })}
                                                        },
                                                        td {
                                                            class: "adui-descriptions-item-content",
                                                            colspan: "{span}",
                                                            {content}
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            {row.iter().map(|item| {
                                                let label = item.label.clone();
                                                let content = item.content.clone();
                                                let span = item.span;
                                                rsx! {
                                                    tr { class: "adui-descriptions-row",
                                                        th {
                                                            class: "adui-descriptions-item-label",
                                                            colspan: "{span * 2}",
                                                            {label}
                                                            {colon.then(|| rsx! { span { class: "adui-descriptions-colon", ":" } })}
                                                        }
                                                    },
                                                    tr { class: "adui-descriptions-row",
                                                        td {
                                                            class: "adui-descriptions-item-content",
                                                            colspan: "{span * 2}",
                                                            {content}
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                })}
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "adui-descriptions-list",
                            {rows.iter().map(|row| {
                                rsx! {
                                    div { class: "adui-descriptions-row",
                                        {row.iter().map(|item| {
                                            let label = item.label.clone();
                                            let content = item.content.clone();
                                            let span = item.span;
                                            let width_percent = (span as f32 / columns as f32 * 100.0) as usize;
                                            if layout == DescriptionsLayout::Horizontal {
                                                rsx! {
                                                    div {
                                                        class: "adui-descriptions-item",
                                                        style: "width: {width_percent}%",
                                                        div {
                                                            class: "adui-descriptions-item-label",
                                                            {label}
                                                            {colon.then(|| rsx! { span { class: "adui-descriptions-colon", ":" } })}
                                                        },
                                                        div { class: "adui-descriptions-item-content",
                                                            {content}
                                                        }
                                                    }
                                                }
                                            } else {
                                                rsx! {
                                                    div {
                                                        class: "adui-descriptions-item adui-descriptions-item-vertical",
                                                        style: "width: {width_percent}%",
                                                        div {
                                                            class: "adui-descriptions-item-label",
                                                            {label}
                                                            {colon.then(|| rsx! { span { class: "adui-descriptions-colon", ":" } })}
                                                        },
                                                        div { class: "adui-descriptions-item-content",
                                                            {content}
                                                        }
                                                    }
                                                }
                                            }
                                        })}
                                    }
                                }
                            })}
                        }
                    }
                }}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptions_size_class_mapping_is_stable() {
        assert_eq!(DescriptionsSize::Small.as_class(), "adui-descriptions-sm");
        assert_eq!(DescriptionsSize::Middle.as_class(), "adui-descriptions-md");
        assert_eq!(DescriptionsSize::Large.as_class(), "adui-descriptions-lg");
    }

    #[test]
    fn descriptions_layout_class_mapping_is_stable() {
        assert_eq!(
            DescriptionsLayout::Horizontal.as_class(),
            "adui-descriptions-horizontal"
        );
        assert_eq!(
            DescriptionsLayout::Vertical.as_class(),
            "adui-descriptions-vertical"
        );
    }

    #[test]
    fn column_config_returns_correct_count() {
        let simple = ColumnConfig::Simple(4);
        assert_eq!(simple.get_columns(), 4);

        let responsive = ColumnConfig::Responsive(ResponsiveColumn::new(3));
        assert_eq!(responsive.get_columns(), 3);
    }
}
