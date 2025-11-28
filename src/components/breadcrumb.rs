use dioxus::prelude::*;

/// Data model for a single breadcrumb item.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    /// Unique identifier for this item. Used when emitting click events.
    pub id: String,
    /// Text label displayed for this breadcrumb node.
    pub label: String,
    /// Optional link target. When present and the item is not the last node,
    /// a clickable `<a>` tag will be rendered.
    pub href: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: None,
        }
    }

    pub fn with_href(
        id: impl Into<String>,
        label: impl Into<String>,
        href: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            href: Some(href.into()),
        }
    }
}

/// Props for the Breadcrumb component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    /// Ordered list of breadcrumb items describing the current path.
    pub items: Vec<BreadcrumbItem>,
    /// Separator string between items. Defaults to `/`.
    #[props(optional)]
    pub separator: Option<String>,
    /// Additional CSS class applied to the breadcrumb root.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline styles applied to the breadcrumb root.
    #[props(optional)]
    pub style: Option<String>,
    /// Optional click handler invoked when a non-final item is clicked.
    ///
    /// The handler receives the `id` of the clicked item. The last item is
    /// treated as the current page and will not trigger this callback.
    #[props(optional)]
    pub on_item_click: Option<EventHandler<String>>,
}

/// Simple Ant Design flavored breadcrumb.
#[component]
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    let BreadcrumbProps {
        items,
        separator,
        class,
        style,
        on_item_click,
    } = props;

    if items.is_empty() {
        return rsx! {};
    }

    let sep = separator.unwrap_or_else(|| "/".to_string());
    let class_attr = {
        let mut list = vec!["adui-breadcrumb".to_string()];
        if let Some(extra) = class {
            list.push(extra);
        }
        list.join(" ")
    };
    let style_attr = style.unwrap_or_default();

    let on_click_cb = on_item_click;
    let total = items.len();

    rsx! {
        nav {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "navigation",
            "aria-label": "Breadcrumb",
            ol {
                class: "adui-breadcrumb-list",
                {items.into_iter().enumerate().map(|(index, item)| {
                    let is_last = index + 1 == total;
                    let id = item.id.clone();
                    let label = item.label.clone();
                    let href = item.href.clone();
                    let sep_text = sep.clone();
                    let on_click_item = on_click_cb;

                    rsx! {
                        li {
                            class: "adui-breadcrumb-item",
                            if !is_last {
                                if let Some(url) = href {
                                    a {
                                        class: "adui-breadcrumb-link",
                                        href: "{url}",
                                        onclick: move |_| {
                                            if let Some(cb) = on_click_item {
                                                cb.call(id.clone());
                                            }
                                        },
                                        "{label}"
                                    }
                                } else {
                                    span {
                                        class: "adui-breadcrumb-text",
                                        onclick: move |_| {
                                            if let Some(cb) = on_click_item {
                                                cb.call(id.clone());
                                            }
                                        },
                                        "{label}"
                                    }
                                }
                                span { class: "adui-breadcrumb-separator", "{sep_text}" }
                            } else {
                                span { class: "adui-breadcrumb-text adui-breadcrumb-text-current", "{label}" }
                            }
                        }
                    }
                })}
            }
        }
    }
}
