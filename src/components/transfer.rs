//! Transfer component for moving items between two lists.
//!
//! A double-column layout component that allows selecting items from a source
//! list and moving them to a target list.

use dioxus::prelude::*;
use std::collections::HashSet;

/// Direction of a Transfer list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferDirection {
    Left,
    Right,
}

/// A single item in the Transfer component.
#[derive(Clone, Debug, PartialEq)]
pub struct TransferItem {
    /// Unique identifier for the item.
    pub key: String,
    /// Display title for the item.
    pub title: String,
    /// Optional description shown below the title.
    pub description: Option<String>,
    /// Whether this item is disabled.
    pub disabled: bool,
}

impl TransferItem {
    /// Create a new transfer item with key and title.
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    /// Builder method to set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Builder method to set disabled state.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Props for the Transfer component.
#[derive(Props, Clone, PartialEq)]
pub struct TransferProps {
    /// Data source for the transfer lists.
    pub data_source: Vec<TransferItem>,
    /// Keys of items that should be in the right (target) list.
    #[props(optional)]
    pub target_keys: Option<Vec<String>>,
    /// Keys of items that are currently selected.
    #[props(optional)]
    pub selected_keys: Option<Vec<String>>,
    /// Titles for the left and right lists.
    #[props(optional)]
    pub titles: Option<(String, String)>,
    /// Text for the transfer operation buttons.
    #[props(optional)]
    pub operations: Option<(String, String)>,
    /// Whether to show search input in the lists.
    #[props(default)]
    pub show_search: bool,
    /// Placeholder text for the search input.
    #[props(optional)]
    pub search_placeholder: Option<String>,
    /// Whether the component is disabled.
    #[props(default)]
    pub disabled: bool,
    /// Whether to show select all checkbox.
    #[props(default = true)]
    pub show_select_all: bool,
    /// One-way mode: items can only be moved from left to right.
    #[props(default)]
    pub one_way: bool,
    /// Callback when target keys change.
    #[props(optional)]
    pub on_change: Option<EventHandler<(Vec<String>, TransferDirection, Vec<String>)>>,
    /// Callback when selection changes.
    #[props(optional)]
    pub on_select_change: Option<EventHandler<(Vec<String>, Vec<String>)>>,
    /// Callback when search text changes.
    #[props(optional)]
    pub on_search: Option<EventHandler<(TransferDirection, String)>>,
    /// Custom filter function for search.
    #[props(optional)]
    pub filter_option: Option<fn(&str, &TransferItem, TransferDirection) -> bool>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
}

/// Transfer component for moving items between two columns.
#[component]
pub fn Transfer(props: TransferProps) -> Element {
    let TransferProps {
        data_source,
        target_keys,
        selected_keys,
        titles,
        operations,
        show_search,
        search_placeholder,
        disabled,
        show_select_all,
        one_way,
        on_change,
        on_select_change,
        on_search,
        filter_option,
        class,
        style,
    } = props;

    // Internal state for target keys if not controlled
    let internal_target_keys: Signal<Vec<String>> = use_signal(Vec::new);
    let current_target_keys = target_keys.unwrap_or_else(|| internal_target_keys.read().clone());

    // Internal state for selected keys
    let internal_selected: Signal<Vec<String>> = use_signal(Vec::new);
    let current_selected = selected_keys.unwrap_or_else(|| internal_selected.read().clone());

    // Search state
    let left_search: Signal<String> = use_signal(String::new);
    let right_search: Signal<String> = use_signal(String::new);

    // Split data into left and right lists (clone to avoid lifetime issues)
    let target_set: HashSet<String> = current_target_keys.iter().cloned().collect();
    let left_items: Vec<TransferItem> = data_source
        .iter()
        .filter(|item| !target_set.contains(&item.key))
        .cloned()
        .collect();
    let right_items: Vec<TransferItem> = data_source
        .iter()
        .filter(|item| target_set.contains(&item.key))
        .cloned()
        .collect();

    // Filter items based on search
    let filter_fn = filter_option.unwrap_or(default_filter);
    let left_search_val = left_search.read().clone();
    let right_search_val = right_search.read().clone();

    let filtered_left: Vec<TransferItem> = if show_search && !left_search_val.is_empty() {
        left_items
            .into_iter()
            .filter(|item| filter_fn(&left_search_val, item, TransferDirection::Left))
            .collect()
    } else {
        left_items
    };

    let filtered_right: Vec<TransferItem> = if show_search && !right_search_val.is_empty() {
        right_items
            .into_iter()
            .filter(|item| filter_fn(&right_search_val, item, TransferDirection::Right))
            .collect()
    } else {
        right_items
    };

    // Selected items in each list
    let selected_set: HashSet<String> = current_selected.iter().cloned().collect();
    let left_selected: Vec<String> = filtered_left
        .iter()
        .filter(|item| selected_set.contains(&item.key) && !item.disabled)
        .map(|item| item.key.clone())
        .collect();
    let right_selected: Vec<String> = filtered_right
        .iter()
        .filter(|item| selected_set.contains(&item.key) && !item.disabled)
        .map(|item| item.key.clone())
        .collect();

    // Titles
    let (left_title, right_title) = titles.unwrap_or(("Source".into(), "Target".into()));
    let (to_right_text, to_left_text) = operations.unwrap_or((">".into(), "<".into()));
    let placeholder = search_placeholder.unwrap_or_else(|| "Search here".into());

    // Build class list
    let mut class_list = vec!["adui-transfer".to_string()];
    if disabled {
        class_list.push("adui-transfer-disabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Handler for moving items to right
    let move_to_right = {
        let current_target = current_target_keys.clone();
        let left_sel = left_selected.clone();
        let on_change = on_change.clone();
        let mut internal_target = internal_target_keys;
        let mut internal_sel = internal_selected;
        move |_| {
            if left_sel.is_empty() || disabled {
                return;
            }
            let mut new_targets = current_target.clone();
            for key in &left_sel {
                if !new_targets.contains(key) {
                    new_targets.push(key.clone());
                }
            }
            let moved = left_sel.clone();
            internal_target.set(new_targets.clone());
            // Clear selection
            internal_sel.set(Vec::new());
            if let Some(handler) = &on_change {
                handler.call((new_targets, TransferDirection::Right, moved));
            }
        }
    };

    // Handler for moving items to left
    let move_to_left = {
        let current_target = current_target_keys.clone();
        let right_sel = right_selected.clone();
        let on_change = on_change.clone();
        let mut internal_target = internal_target_keys;
        let mut internal_sel = internal_selected;
        move |_| {
            if right_sel.is_empty() || disabled || one_way {
                return;
            }
            let sel_set: HashSet<&str> = right_sel.iter().map(|s| s.as_str()).collect();
            let new_targets: Vec<String> = current_target
                .iter()
                .filter(|k| !sel_set.contains(k.as_str()))
                .cloned()
                .collect();
            let moved = right_sel.clone();
            internal_target.set(new_targets.clone());
            internal_sel.set(Vec::new());
            if let Some(handler) = &on_change {
                handler.call((new_targets, TransferDirection::Left, moved));
            }
        }
    };

    // Item selection handler
    let handle_select = {
        let current_sel = current_selected.clone();
        let on_select_change = on_select_change.clone();
        let mut internal_sel = internal_selected;
        let target_set = target_set.clone();
        move |key: String| {
            if disabled {
                return;
            }
            let mut new_selected = current_sel.clone();
            if new_selected.contains(&key) {
                new_selected.retain(|k| k != &key);
            } else {
                new_selected.push(key.clone());
            }

            // Calculate left and right selections
            let left_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| !target_set.contains(*k))
                .cloned()
                .collect();
            let right_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| target_set.contains(*k))
                .cloned()
                .collect();

            internal_sel.set(new_selected);
            if let Some(handler) = &on_select_change {
                handler.call((left_sel, right_sel));
            }
        }
    };

    // Select all handler for a direction
    let handle_select_all_left = {
        let current_sel = current_selected.clone();
        let filtered_left = filtered_left.clone();
        let on_select_change = on_select_change.clone();
        let mut internal_sel = internal_selected;
        let target_set_clone = target_set.clone();
        move |select_all: bool| {
            if disabled {
                return;
            }
            let item_keys: HashSet<String> = filtered_left
                .iter()
                .filter(|i| !i.disabled)
                .map(|i| i.key.clone())
                .collect();

            let mut new_selected: Vec<String> = current_sel
                .iter()
                .filter(|k| !item_keys.contains(*k))
                .cloned()
                .collect();

            if select_all {
                for key in item_keys {
                    new_selected.push(key);
                }
            }

            let left_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| !target_set_clone.contains(*k))
                .cloned()
                .collect();
            let right_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| target_set_clone.contains(*k))
                .cloned()
                .collect();

            internal_sel.set(new_selected);
            if let Some(handler) = &on_select_change {
                handler.call((left_sel, right_sel));
            }
        }
    };

    let handle_select_all_right = {
        let current_sel = current_selected.clone();
        let filtered_right = filtered_right.clone();
        let on_select_change = on_select_change.clone();
        let mut internal_sel = internal_selected;
        let target_set_clone = target_set.clone();
        move |select_all: bool| {
            if disabled {
                return;
            }
            let item_keys: HashSet<String> = filtered_right
                .iter()
                .filter(|i| !i.disabled)
                .map(|i| i.key.clone())
                .collect();

            let mut new_selected: Vec<String> = current_sel
                .iter()
                .filter(|k| !item_keys.contains(*k))
                .cloned()
                .collect();

            if select_all {
                for key in item_keys {
                    new_selected.push(key);
                }
            }

            let left_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| !target_set_clone.contains(*k))
                .cloned()
                .collect();
            let right_sel: Vec<String> = new_selected
                .iter()
                .filter(|k| target_set_clone.contains(*k))
                .cloned()
                .collect();

            internal_sel.set(new_selected);
            if let Some(handler) = &on_select_change {
                handler.call((left_sel, right_sel));
            }
        }
    };

    // Search handlers
    let on_left_search = {
        let mut search = left_search;
        let on_search = on_search.clone();
        move |evt: Event<FormData>| {
            let value = evt.value();
            search.set(value.clone());
            if let Some(handler) = &on_search {
                handler.call((TransferDirection::Left, value));
            }
        }
    };

    let on_right_search = {
        let mut search = right_search;
        let on_search = on_search.clone();
        move |evt: Event<FormData>| {
            let value = evt.value();
            search.set(value.clone());
            if let Some(handler) = &on_search {
                handler.call((TransferDirection::Right, value));
            }
        }
    };

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            // Left list
            TransferList {
                title: left_title,
                items: filtered_left.clone(),
                selected_keys: left_selected.clone(),
                disabled: disabled,
                show_search: show_search,
                search_placeholder: placeholder.clone(),
                search_value: left_search_val.clone(),
                on_search: on_left_search,
                on_select: handle_select.clone(),
                on_select_all: handle_select_all_left,
                show_select_all: show_select_all,
            }

            // Operations
            div { class: "adui-transfer-operations",
                button {
                    class: "adui-transfer-operation-btn",
                    r#type: "button",
                    disabled: left_selected.is_empty() || disabled,
                    onclick: move_to_right,
                    "{to_right_text}"
                }
                if !one_way {
                    button {
                        class: "adui-transfer-operation-btn",
                        r#type: "button",
                        disabled: right_selected.is_empty() || disabled,
                        onclick: move_to_left,
                        "{to_left_text}"
                    }
                }
            }

            // Right list
            TransferList {
                title: right_title,
                items: filtered_right.clone(),
                selected_keys: right_selected.clone(),
                disabled: disabled,
                show_search: show_search,
                search_placeholder: placeholder.clone(),
                search_value: right_search_val.clone(),
                on_search: on_right_search,
                on_select: handle_select.clone(),
                on_select_all: handle_select_all_right,
                show_select_all: show_select_all,
            }
        }
    }
}

/// Default filter function for search.
fn default_filter(query: &str, item: &TransferItem, _direction: TransferDirection) -> bool {
    let query_lower = query.to_lowercase();
    item.title.to_lowercase().contains(&query_lower)
        || item
            .description
            .as_ref()
            .map(|d| d.to_lowercase().contains(&query_lower))
            .unwrap_or(false)
}

/// Props for the internal TransferList component.
#[derive(Props, Clone, PartialEq)]
struct TransferListProps {
    title: String,
    items: Vec<TransferItem>,
    selected_keys: Vec<String>,
    disabled: bool,
    show_search: bool,
    search_placeholder: String,
    search_value: String,
    on_search: EventHandler<Event<FormData>>,
    on_select: EventHandler<String>,
    on_select_all: EventHandler<bool>,
    show_select_all: bool,
}

/// Internal list component for one side of the Transfer.
#[component]
fn TransferList(props: TransferListProps) -> Element {
    let TransferListProps {
        title,
        items,
        selected_keys,
        disabled,
        show_search,
        search_placeholder,
        search_value,
        on_search,
        on_select,
        on_select_all,
        show_select_all,
    } = props;

    let selected_set: HashSet<&str> = selected_keys.iter().map(|s| s.as_str()).collect();
    let selectable_count = items.iter().filter(|i| !i.disabled).count();
    let selected_count = selected_keys.len();
    let all_selected = selectable_count > 0 && selected_count == selectable_count;
    let some_selected = selected_count > 0 && selected_count < selectable_count;

    let mut header_checkbox_class = vec!["adui-transfer-list-header-checkbox".to_string()];
    if all_selected {
        header_checkbox_class.push("adui-checkbox-checked".into());
    } else if some_selected {
        header_checkbox_class.push("adui-checkbox-indeterminate".into());
    }

    rsx! {
        div { class: "adui-transfer-list",
            // Header
            div { class: "adui-transfer-list-header",
                if show_select_all {
                    span {
                        class: "{header_checkbox_class.join(\" \")}",
                        onclick: move |_| {
                            if !disabled {
                                on_select_all.call(!all_selected);
                            }
                        },
                        span { class: "adui-checkbox-inner" }
                    }
                }
                span { class: "adui-transfer-list-header-selected",
                    "{selected_count}/{items.len()} items"
                }
                span { class: "adui-transfer-list-header-title", "{title}" }
            }

            // Search
            if show_search {
                div { class: "adui-transfer-list-search",
                    input {
                        class: "adui-input",
                        r#type: "text",
                        placeholder: "{search_placeholder}",
                        value: "{search_value}",
                        disabled: disabled,
                        oninput: move |evt| on_search.call(evt),
                    }
                }
            }

            // Body
            div { class: "adui-transfer-list-body",
                ul { class: "adui-transfer-list-content",
                    for item in items.iter() {
                        TransferListItem {
                            key: "{item.key}",
                            item: item.clone(),
                            selected: selected_set.contains(item.key.as_str()),
                            disabled: disabled || item.disabled,
                            on_select: on_select.clone(),
                        }
                    }
                    if items.is_empty() {
                        li { class: "adui-transfer-list-empty", "No data" }
                    }
                }
            }
        }
    }
}

/// Props for a single transfer list item.
#[derive(Props, Clone, PartialEq)]
struct TransferListItemProps {
    item: TransferItem,
    selected: bool,
    disabled: bool,
    on_select: EventHandler<String>,
}

/// Single item in the transfer list.
#[component]
fn TransferListItem(props: TransferListItemProps) -> Element {
    let TransferListItemProps {
        item,
        selected,
        disabled,
        on_select,
    } = props;

    let mut class_list = vec!["adui-transfer-list-item".to_string()];
    if selected {
        class_list.push("adui-transfer-list-item-selected".into());
    }
    if disabled {
        class_list.push("adui-transfer-list-item-disabled".into());
    }

    let mut checkbox_class = vec!["adui-checkbox".to_string()];
    if selected {
        checkbox_class.push("adui-checkbox-checked".into());
    }
    if disabled {
        checkbox_class.push("adui-checkbox-disabled".into());
    }

    let key = item.key.clone();

    rsx! {
        li {
            class: "{class_list.join(\" \")}",
            onclick: move |_| {
                if !disabled {
                    on_select.call(key.clone());
                }
            },
            span { class: "{checkbox_class.join(\" \")}",
                span { class: "adui-checkbox-inner" }
            }
            span { class: "adui-transfer-list-item-content",
                span { class: "adui-transfer-list-item-title", "{item.title}" }
                if let Some(desc) = &item.description {
                    span { class: "adui-transfer-list-item-description", "{desc}" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_item_builder_works() {
        let item = TransferItem::new("key1", "Title 1")
            .with_description("Description")
            .with_disabled(true);
        assert_eq!(item.key, "key1");
        assert_eq!(item.title, "Title 1");
        assert_eq!(item.description, Some("Description".into()));
        assert!(item.disabled);
    }

    #[test]
    fn default_filter_matches_title() {
        let item = TransferItem::new("1", "Hello World");
        assert!(default_filter("hello", &item, TransferDirection::Left));
        assert!(default_filter("WORLD", &item, TransferDirection::Left));
        assert!(!default_filter("xyz", &item, TransferDirection::Left));
    }

    #[test]
    fn default_filter_matches_description() {
        let item = TransferItem::new("1", "Title").with_description("Some description here");
        assert!(default_filter(
            "description",
            &item,
            TransferDirection::Right
        ));
        assert!(!default_filter("notfound", &item, TransferDirection::Right));
    }
}
