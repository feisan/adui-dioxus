use dioxus::prelude::*;

/// Props for the Pagination component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Controlled current page index (1-based).
    #[props(optional)]
    pub current: Option<u32>,
    /// Initial page index when uncontrolled.
    #[props(optional)]
    pub default_current: Option<u32>,
    /// Controlled page size.
    #[props(optional)]
    pub page_size: Option<u32>,
    /// Initial page size when uncontrolled.
    #[props(optional)]
    pub default_page_size: Option<u32>,
    /// Total number of items.
    pub total: u32,
    /// Optional page size options for the size changer.
    #[props(optional)]
    pub page_size_options: Option<Vec<u32>>,
    /// Whether to show the page size changer dropdown.
    #[props(default)]
    pub show_size_changer: bool,
    /// Whether to display a simple total text (e.g. "共 X 条").
    #[props(default)]
    pub show_total: bool,
    /// Called when page or page size changes due to user interaction.
    /// Argument is `(page, page_size)`.
    #[props(optional)]
    pub on_change: Option<EventHandler<(u32, u32)>>,
    /// Called specifically when the page size changes.
    #[props(optional)]
    pub on_page_size_change: Option<EventHandler<(u32, u32)>>,
    /// Extra class for the pagination root.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the pagination root.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Pagination (MVP).
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let PaginationProps {
        current,
        default_current,
        page_size,
        default_page_size,
        total,
        page_size_options,
        show_size_changer,
        show_total,
        on_change,
        on_page_size_change,
        class,
        style,
    } = props;

    let default_page_size_value = default_page_size.unwrap_or(10).max(1);
    let page_size_internal: Signal<u32> = use_signal(|| default_page_size_value);
    let current_internal: Signal<u32> = use_signal(|| default_current.unwrap_or(1).max(1));

    let is_page_controlled = current.is_some();
    let is_size_controlled = page_size.is_some();

    let page_size_value = page_size.unwrap_or(*page_size_internal.read());
    let total_pages = total.div_ceil(page_size_value).max(1);
    let current_value = current.unwrap_or_else(|| current_internal.read().clamp(1, total_pages));

    let class_attr = {
        let mut list = vec!["adui-pagination".to_string()];
        if let Some(extra) = class {
            list.push(extra);
        }
        list.join(" ")
    };
    let style_attr = style.unwrap_or_default();

    let on_change_cb = on_change;
    let on_page_size_change_cb = on_page_size_change;
    let current_signal = current_internal;
    let size_signal = page_size_internal;

    let update_state = move |next_page: u32, next_size: u32, emit_size_change: bool| {
        let next_page = next_page.clamp(1, total.div_ceil(next_size).max(1));

        if !is_page_controlled {
            let mut sig = current_signal;
            sig.set(next_page);
        }
        if !is_size_controlled {
            let mut sig = size_signal;
            sig.set(next_size);
        }

        if let Some(cb) = on_change_cb {
            cb.call((next_page, next_size));
        }
        if emit_size_change && let Some(cb) = on_page_size_change_cb {
            cb.call((next_page, next_size));
        }
    };

    let render_pages = |current_page: u32, total_pages: u32| -> Vec<u32> {
        let mut pages = Vec::new();
        if total_pages <= 7 {
            for p in 1..=total_pages {
                pages.push(p);
            }
        } else {
            pages.push(1);
            if current_page > 4 {
                pages.push(0); // 0 represents an ellipsis
            }
            let start = current_page.saturating_sub(1).max(2);
            let end = (current_page + 1).min(total_pages - 1);
            for p in start..=end {
                pages.push(p);
            }
            if current_page + 2 < total_pages {
                pages.push(0);
            }
            pages.push(total_pages);
        }
        pages
    };

    let pages = render_pages(current_value, total_pages);

    // Pre-compute page size options and an update callback for the size changer.
    let size_changer_options = page_size_options.unwrap_or_else(|| vec![10, 20, 50]);
    let update_for_size = update_state;

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if show_total {
                span { class: "adui-pagination-total", "共 {total} 条" }
            }

            // Page items
            ul { class: "adui-pagination-list",
                // Prev
                {
                    let disabled = current_value <= 1;
                    let update = update_state;
                    rsx!(
                        li {
                            class: {
                                let mut c = vec!["adui-pagination-item".to_string()];
                                if disabled { c.push("adui-pagination-item-disabled".into()); }
                                c.join(" ")
                            },
                            onclick: move |_| {
                                if !disabled {
                                    update(current_value.saturating_sub(1), page_size_value, false);
                                }
                            },
                            "上一页"
                        }
                    )
                }

                {pages.into_iter().map(|p| {
                    if p == 0 {
                        rsx! { li { class: "adui-pagination-item adui-pagination-item-ellipsis", "..." } }
                    } else {
                        let is_active = p == current_value;
                        let update = update_state;
                        rsx! {
                            li {
                                class: {
                                    let mut c = vec!["adui-pagination-item".to_string()];
                                    if is_active { c.push("adui-pagination-item-active".into()); }
                                    c.join(" ")
                                },
                                onclick: move |_| {
                                    if !is_active {
                                        update(p, page_size_value, false);
                                    }
                                },
                                "{p}"
                            }
                        }
                    }
                })}

                // Next
                {
                    let disabled = current_value >= total_pages;
                    let update = update_state;
                    rsx!(
                        li {
                            class: {
                                let mut c = vec!["adui-pagination-item".to_string()];
                                if disabled { c.push("adui-pagination-item-disabled".into()); }
                                c.join(" ")
                            },
                            onclick: move |_| {
                                if !disabled {
                                    update(current_value + 1, page_size_value, false);
                                }
                            },
                            "下一页"
                        }
                    )
                }
            }

            if show_size_changer {
                select {
                    class: "adui-pagination-size-changer",
                    value: format!("{page_size_value}"),
                    onchange: move |evt| {
                        if let Ok(next) = evt.value().parse::<u32>() {
                            update_for_size(current_value, next, true);
                        }
                    },
                    {size_changer_options.into_iter().map(|opt| {
                        rsx! {
                            option {
                                value: format!("{opt}"),
                                "{opt} / 页"
                            }
                        }
                    })}
                }
            }
        }
    }
}
