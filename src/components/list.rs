use crate::components::config_provider::ComponentSize;
use crate::components::empty::Empty;
use crate::components::pagination::Pagination;
use crate::components::spin::Spin;
use dioxus::prelude::*;

/// Props for the List component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    /// Optional header content displayed above the list items.
    #[props(optional)]
    pub header: Option<Element>,
    /// Optional footer content displayed below the list body.
    #[props(optional)]
    pub footer: Option<Element>,
    /// Whether to render a border around the list.
    #[props(default)]
    pub bordered: bool,
    /// Visual density. When set, maps to size-specific classes.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Extra class name for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Whether the list is in loading state. When true, the body is wrapped
    /// with a Spin overlay.
    #[props(default)]
    pub loading: bool,
    /// Whether the data set is empty (used together with `loading=false`).
    #[props(optional)]
    pub is_empty: Option<bool>,
    /// Custom empty content. When omitted and `is_empty = Some(true)`, the
    /// built-in `Empty` component is used.
    #[props(optional)]
    pub empty: Option<Element>,
    /// Pagination total items. When set, enables a simple Pagination control
    /// at the bottom of the list.
    #[props(optional)]
    pub pagination_total: Option<u32>,
    /// Current page index for pagination (1-based).
    #[props(optional)]
    pub pagination_current: Option<u32>,
    /// Page size for pagination.
    #[props(optional)]
    pub pagination_page_size: Option<u32>,
    /// Callback when pagination changes.
    #[props(optional)]
    pub pagination_on_change: Option<EventHandler<(u32, u32)>>,
    /// List item content. Callers通常在内部渲染多个块元素，并使用
    /// `.adui-list-item` 类名标记每一行。
    pub children: Element,
}

/// Ant Design flavored List component (MVP).
#[component]
pub fn List(props: ListProps) -> Element {
    let ListProps {
        header,
        footer,
        bordered,
        size,
        class,
        style,
        loading,
        is_empty,
        empty,
        pagination_total,
        pagination_current,
        pagination_page_size,
        pagination_on_change,
        children,
    } = props;

    let mut class_list = vec!["adui-list".to_string()];
    if bordered {
        class_list.push("adui-list-bordered".into());
    }
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-list-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-list-lg".into()),
        }
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let show_empty = !loading && is_empty.unwrap_or(false);

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if let Some(head) = header {
                div { class: "adui-list-header", {head} }
            }

            // Body
            if loading {
                div { class: "adui-list-body",
                    Spin {
                        spinning: Some(true),
                        tip: Some("加载中...".to_string()),
                        div { class: "adui-list-items", {children} }
                    }
                }
            } else if show_empty {
                div { class: "adui-list-body",
                    div { class: "adui-list-empty",
                        if let Some(node) = empty {
                            {node}
                        } else {
                            Empty {}
                        }
                    }
                }
            } else {
                div { class: "adui-list-body",
                    div { class: "adui-list-items", {children} }
                }
            }

            if let Some(foot) = footer {
                div { class: "adui-list-footer", {foot} }
            }

            // Pagination
            if let Some(total) = pagination_total {
                div { class: "adui-list-pagination",
                    Pagination {
                        total: total,
                        current: pagination_current,
                        page_size: pagination_page_size,
                        show_total: false,
                        show_size_changer: false,
                        on_change: move |(page, size)| {
                            if let Some(cb) = pagination_on_change {
                                cb.call((page, size));
                            }
                        },
                    }
                }
            }
        }
    }
}
