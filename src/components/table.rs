//! Table component aligned with Ant Design 6.0.
//!
//! Features:
//! - Custom column render functions
//! - Row selection (checkbox column)
//! - Sorting and filtering
//! - Fixed header/columns via scroll
//! - Pagination integration

use crate::components::checkbox::Checkbox;
use crate::components::config_provider::ComponentSize;
use crate::components::empty::Empty;
use crate::components::icon::{Icon, IconKind};
use crate::components::pagination::Pagination;
use crate::components::spin::Spin;
use crate::foundation::{
    ClassListExt, StyleStringExt, TableClassNames, TableSemantic, TableStyles,
};
use dioxus::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;

/// Horizontal alignment for table cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColumnAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl ColumnAlign {
    fn as_class(&self) -> &'static str {
        match self {
            ColumnAlign::Left => "adui-table-align-left",
            ColumnAlign::Center => "adui-table-align-center",
            ColumnAlign::Right => "adui-table-align-right",
        }
    }
}

/// Sort direction for a column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortOrder {
    Ascend,
    Descend,
}

impl SortOrder {
    fn as_class(&self) -> &'static str {
        match self {
            SortOrder::Ascend => "adui-table-column-sort-ascend",
            SortOrder::Descend => "adui-table-column-sort-descend",
        }
    }
}

/// Fixed position for a column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnFixed {
    Left,
    Right,
}

/// Type alias for custom render function.
/// Takes (value, record, index) and returns Element.
pub type ColumnRenderFn = fn(Option<&Value>, &Value, usize) -> Element;

/// Filter item for column filters.
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnFilter {
    pub text: String,
    pub value: String,
}

/// Type alias for filter function.
/// Takes (value, record) and returns whether to show the row.
pub type ColumnFilterFn = fn(&str, &Value) -> bool;

/// Type alias for sort compare function.
/// Takes (a, b) and returns ordering.
pub type ColumnSorterFn = fn(&Value, &Value) -> std::cmp::Ordering;

/// Column definition for the Table component.
#[derive(Clone)]
pub struct TableColumn {
    pub key: String,
    pub title: String,
    pub data_index: Option<String>,
    pub width: Option<f32>,
    pub align: Option<ColumnAlign>,
    pub fixed: Option<ColumnFixed>,
    /// Whether this column can be sorted.
    pub sortable: bool,
    /// Default sort order.
    pub default_sort_order: Option<SortOrder>,
    /// Custom sorter function.
    #[allow(clippy::type_complexity)]
    pub sorter: Option<ColumnSorterFn>,
    /// Filter options.
    pub filters: Option<Vec<ColumnFilter>>,
    /// Custom filter function.
    #[allow(clippy::type_complexity)]
    pub on_filter: Option<ColumnFilterFn>,
    /// Custom render function.
    #[allow(clippy::type_complexity)]
    pub render: Option<ColumnRenderFn>,
    /// Whether column is hidden.
    pub hidden: bool,
    /// Ellipsis text overflow.
    pub ellipsis: bool,
}

impl PartialEq for TableColumn {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.title == other.title
            && self.data_index == other.data_index
            && self.width == other.width
            && self.align == other.align
            && self.fixed == other.fixed
            && self.sortable == other.sortable
            && self.default_sort_order == other.default_sort_order
            && self.filters == other.filters
            && self.hidden == other.hidden
            && self.ellipsis == other.ellipsis
    }
}

impl TableColumn {
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        let key_str = key.into();
        Self {
            key: key_str.clone(),
            title: title.into(),
            data_index: Some(key_str),
            width: None,
            align: None,
            fixed: None,
            sortable: false,
            default_sort_order: None,
            sorter: None,
            filters: None,
            on_filter: None,
            render: None,
            hidden: false,
            ellipsis: false,
        }
    }

    /// Set data index for this column.
    pub fn data_index(mut self, index: impl Into<String>) -> Self {
        self.data_index = Some(index.into());
        self
    }

    /// Set width for this column.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set alignment for this column.
    pub fn align(mut self, align: ColumnAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Set this column as sortable.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// Set custom render function.
    pub fn render(mut self, render: ColumnRenderFn) -> Self {
        self.render = Some(render);
        self
    }

    /// Set fixed position.
    pub fn fixed(mut self, fixed: ColumnFixed) -> Self {
        self.fixed = Some(fixed);
        self
    }

    /// Set ellipsis overflow.
    pub fn ellipsis(mut self) -> Self {
        self.ellipsis = true;
        self
    }
}

/// Row selection configuration.
#[derive(Clone, Default, PartialEq)]
pub struct RowSelection {
    /// Selected row keys.
    pub selected_row_keys: Vec<String>,
    /// Callback when selection changes.
    pub on_change: Option<EventHandler<Vec<String>>>,
    /// Selection type (checkbox or radio).
    pub selection_type: SelectionType,
    /// Whether to preserve selection when data changes.
    pub preserve_selected_row_keys: bool,
}

/// Selection type for row selection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionType {
    #[default]
    Checkbox,
    Radio,
}

/// Expandable row configuration.
#[derive(Clone)]
pub struct ExpandableConfig {
    /// Whether rows are expandable by default.
    pub expanded_row_keys: Vec<String>,
    /// Callback when expanded rows change.
    pub on_expand: Option<EventHandler<(bool, String)>>,
    /// Custom expand icon.
    pub expand_icon: Option<Element>,
    /// Custom expanded row render function: (record, index, indent, expanded) -> Element
    pub expanded_row_render: Option<Rc<dyn Fn(&Value, usize, usize, bool) -> Element>>,
    /// Whether to show expand icon for all rows (even if expanded_row_render returns None).
    pub show_expand_icon: bool,
}

impl Default for ExpandableConfig {
    fn default() -> Self {
        Self {
            expanded_row_keys: Vec::new(),
            on_expand: None,
            expand_icon: None,
            expanded_row_render: None,
            show_expand_icon: true,
        }
    }
}

impl PartialEq for ExpandableConfig {
    fn eq(&self, other: &Self) -> bool {
        self.expanded_row_keys == other.expanded_row_keys
            && self.show_expand_icon == other.show_expand_icon
        // Functions cannot be compared
    }
}

/// Scroll configuration for fixed header/columns.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableScroll {
    /// Fixed table height (for virtual scrolling or fixed header).
    pub y: Option<f32>,
    /// Fixed table width (for horizontal scrolling).
    pub x: Option<f32>,
    /// Whether to scroll to top when data changes.
    pub scroll_to_first_row_on_change: bool,
}

/// Sticky configuration for table header.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StickyConfig {
    /// Offset from top when sticky.
    pub offset_top: Option<f32>,
    /// Offset from bottom when sticky.
    pub offset_bottom: Option<f32>,
    /// Container element selector for sticky calculation.
    pub get_container: Option<String>,
}

/// Summary row configuration.
#[derive(Clone)]
pub struct SummaryConfig {
    /// Custom summary row render function: (columns, data) -> Element
    pub render: Option<Rc<dyn Fn(&[TableColumn], &[Value]) -> Element>>,
    /// Fixed position for summary row.
    pub fixed: Option<ColumnFixed>,
}

impl PartialEq for SummaryConfig {
    fn eq(&self, other: &Self) -> bool {
        self.fixed == other.fixed
        // Functions cannot be compared
    }
}

/// Type alias for row class name function: (record, index) -> Option<String>
pub type RowClassNameFn = Rc<dyn Fn(&Value, usize) -> Option<String>>;

/// Type alias for row props function: (record, index) -> HashMap<String, String>
pub type RowPropsFn = Rc<dyn Fn(&Value, usize) -> HashMap<String, String>>;

/// Locale configuration for table text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableLocale {
    /// Text for filter title.
    pub filter_title: Option<String>,
    /// Text for filter confirm button.
    pub filter_confirm: Option<String>,
    /// Text for filter reset button.
    pub filter_reset: Option<String>,
    /// Text when filter is empty.
    pub filter_empty_text: Option<String>,
    /// Text for "select all".
    pub select_all: Option<String>,
    /// Text for "select none".
    pub select_none: Option<String>,
    /// Text for "select invert".
    pub select_invert: Option<String>,
    /// Text for sort title.
    pub sort_title: Option<String>,
    /// Text for expand.
    pub expand: Option<String>,
    /// Text for collapse.
    pub collapse: Option<String>,
    /// Text for empty table.
    pub empty_text: Option<String>,
}

/// Event payload for table changes (pagination, filters, sorter).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableChangeEvent {
    pub pagination: Option<TablePaginationState>,
    pub sorter: Option<TableSorterState>,
    pub filters: HashMap<String, Vec<String>>,
}

/// Pagination state in change event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TablePaginationState {
    pub current: u32,
    pub page_size: u32,
    pub total: u32,
}

/// Sorter state in change event.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableSorterState {
    pub column_key: Option<String>,
    pub order: Option<SortOrder>,
}

/// Props for the Table component.
#[derive(Props, Clone)]
pub struct TableProps {
    /// Column definitions.
    pub columns: Vec<TableColumn>,
    /// Row data as JSON values.
    pub data: Vec<Value>,
    /// Field used as row key.
    #[props(optional)]
    pub row_key_field: Option<String>,
    /// Extra class applied to each row (static).
    #[props(optional)]
    pub row_class_name: Option<String>,
    /// Dynamic row class name function: (record, index) -> Option<String>
    #[props(optional)]
    pub row_class_name_fn: Option<RowClassNameFn>,
    /// Dynamic row props function: (record, index) -> HashMap<String, String>
    #[props(optional)]
    pub row_props_fn: Option<RowPropsFn>,
    /// Whether to show outer borders.
    #[props(default)]
    pub bordered: bool,
    /// Visual density.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Loading state.
    #[props(default)]
    pub loading: bool,
    /// Whether the table is currently empty.
    #[props(optional)]
    pub is_empty: Option<bool>,
    /// Custom empty node.
    #[props(optional)]
    pub empty: Option<Element>,
    /// Row selection configuration.
    #[props(optional)]
    pub row_selection: Option<RowSelection>,
    /// Scroll configuration.
    #[props(optional)]
    pub scroll: Option<TableScroll>,
    /// Sticky header configuration.
    #[props(optional)]
    pub sticky: Option<StickyConfig>,
    /// Expandable row configuration.
    #[props(optional)]
    pub expandable: Option<ExpandableConfig>,
    /// Summary row configuration.
    #[props(optional)]
    pub summary: Option<SummaryConfig>,
    /// Called when pagination, filters or sorter changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<TableChangeEvent>>,
    /// Custom container for popups (dropdowns, filters, etc.).
    /// Function that takes trigger node and returns container element.
    /// In Rust, this is simplified to a container selector string.
    #[props(optional)]
    pub get_popup_container: Option<String>,
    /// Enable virtual scrolling for large datasets.
    #[props(default)]
    pub r#virtual: bool,
    /// Locale configuration for table text.
    #[props(optional)]
    pub locale: Option<TableLocale>,
    /// Show table header.
    #[props(default = true)]
    pub show_header: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names.
    #[props(optional)]
    pub class_names: Option<TableClassNames>,
    /// Semantic styles.
    #[props(optional)]
    pub styles: Option<TableStyles>,
    // Pagination props (simplified)
    #[props(optional)]
    pub pagination_total: Option<u32>,
    #[props(optional)]
    pub pagination_current: Option<u32>,
    #[props(optional)]
    pub pagination_page_size: Option<u32>,
    #[props(optional)]
    pub pagination_on_change: Option<EventHandler<(u32, u32)>>,
}

impl PartialEq for TableProps {
    fn eq(&self, other: &Self) -> bool {
        // Compare all fields except function pointers
        self.columns == other.columns
            && self.data == other.data
            && self.row_key_field == other.row_key_field
            && self.row_class_name == other.row_class_name
            && self.bordered == other.bordered
            && self.size == other.size
            && self.loading == other.loading
            && self.is_empty == other.is_empty
            && self.empty == other.empty
            && self.row_selection == other.row_selection
            && self.scroll == other.scroll
            && self.sticky == other.sticky
            && self.expandable == other.expandable
            && self.summary == other.summary
            && self.on_change == other.on_change
            && self.show_header == other.show_header
            && self.class == other.class
            && self.style == other.style
            && self.class_names == other.class_names
            && self.styles == other.styles
            && self.get_popup_container == other.get_popup_container
            && self.r#virtual == other.r#virtual
            && self.locale == other.locale
            && self.pagination_total == other.pagination_total
            && self.pagination_current == other.pagination_current
            && self.pagination_page_size == other.pagination_page_size
        // Function pointers cannot be compared for equality
    }
}

/// Ant Design flavored Table.
#[component]
pub fn Table(props: TableProps) -> Element {
    let TableProps {
        columns,
        data,
        row_key_field,
        row_class_name,
        row_class_name_fn,
        row_props_fn,
        bordered,
        size,
        loading,
        is_empty,
        empty,
        row_selection,
        scroll,
        sticky,
        expandable,
        summary,
        on_change,
        show_header,
        class,
        style,
        class_names,
        styles,
        get_popup_container,
        r#virtual: virtual_scrolling,
        locale,
        pagination_total,
        pagination_current,
        pagination_page_size,
        pagination_on_change,
    } = props;

    // Internal sort state
    let sort_state: Signal<Option<(String, SortOrder)>> = use_signal(|| None);

    // Filter visible columns
    let visible_columns: Vec<&TableColumn> = columns.iter().filter(|c| !c.hidden).collect();

    // Build table classes
    let mut class_list = vec!["adui-table".to_string()];
    if bordered {
        class_list.push("adui-table-bordered".into());
    }
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-table-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-table-lg".into()),
        }
    }
    if row_selection.is_some() {
        class_list.push("adui-table-selection".into());
    }
    class_list.push_semantic(&class_names, TableSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut style_attr = style.unwrap_or_default();
    style_attr.append_semantic(&styles, TableSemantic::Root);

    // Scroll styles
    let scroll_style = if let Some(ref sc) = scroll {
        let mut s = String::new();
        if let Some(y) = sc.y {
            s.push_str(&format!("max-height: {}px; overflow-y: auto;", y));
        }
        if let Some(x) = sc.x {
            s.push_str(&format!("overflow-x: auto; min-width: {}px;", x));
        }
        s
    } else {
        String::new()
    };

    let show_empty = !loading && is_empty.unwrap_or(data.is_empty());

    // Row key helper
    let get_row_key = |row: &Value, idx: usize| -> String {
        if let Some(field) = &row_key_field {
            get_cell_text(row, field)
        } else {
            idx.to_string()
        }
    };

    // Selection helpers
    let has_selection = row_selection.is_some();
    let selection_type = row_selection
        .as_ref()
        .map(|r| r.selection_type)
        .unwrap_or_default();
    let selected_keys = row_selection
        .as_ref()
        .map(|r| r.selected_row_keys.clone())
        .unwrap_or_default();

    let all_keys: Vec<String> = data
        .iter()
        .enumerate()
        .map(|(idx, row)| get_row_key(row, idx))
        .collect();

    let all_selected = !all_keys.is_empty() && all_keys.iter().all(|k| selected_keys.contains(k));
    let some_selected = !selected_keys.is_empty() && !all_selected;

    // Selection change handler
    let on_select_change = row_selection.as_ref().and_then(|r| r.on_change);

    // Sort handler
    let on_change_cb = on_change;
    let pagination_total_for_change = pagination_total;
    let pagination_current_for_change = pagination_current;
    let pagination_page_size_for_change = pagination_page_size;

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if show_header {
                div { class: "adui-table-header",
                    div { class: "adui-table-row adui-table-row-header",
                        // Selection column header
                        if has_selection {
                            div { class: "adui-table-cell adui-table-cell-selection",
                                if matches!(selection_type, SelectionType::Checkbox) {
                                    Checkbox {
                                        checked: all_selected,
                                        indeterminate: some_selected,
                                        on_change: move |_| {
                                            if let Some(cb) = on_select_change {
                                                if all_selected {
                                                    cb.call(Vec::new());
                                                } else {
                                                    cb.call(all_keys.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Data columns
                        {visible_columns.iter().map(|col| {
                            let mut cell_classes = vec!["adui-table-cell".to_string(), "adui-table-cell-header".to_string()];
                            if let Some(align) = col.align {
                                cell_classes.push(align.as_class().to_string());
                            }
                            if col.sortable {
                                cell_classes.push("adui-table-column-sortable".into());
                            }
                            if let Some((ref key, order)) = *sort_state.read() {
                                if key == &col.key {
                                    cell_classes.push(order.as_class().to_string());
                                }
                            }
                            if col.ellipsis {
                                cell_classes.push("adui-table-cell-ellipsis".into());
                            }

                            let width_style = col.width.map(|w| format!("width: {}px;", w)).unwrap_or_default();
                            let title = col.title.clone();
                            let cell_class = cell_classes.join(" ");

                            let col_key = col.key.clone();
                            let sortable = col.sortable;
                            let mut sort_signal = sort_state;

                            rsx! {
                                div {
                                    class: "{cell_class}",
                                    style: "{width_style}",
                                    onclick: move |_| {
                                        if sortable {
                                            let current = sort_signal.read().clone();
                                            let new_order = match current {
                                                Some((ref k, SortOrder::Ascend)) if k == &col_key => {
                                                    Some((col_key.clone(), SortOrder::Descend))
                                                }
                                                Some((ref k, SortOrder::Descend)) if k == &col_key => {
                                                    None
                                                }
                                                _ => Some((col_key.clone(), SortOrder::Ascend))
                                            };
                                            sort_signal.set(new_order.clone());

                                            // Emit change event
                                            if let Some(cb) = on_change_cb {
                                                cb.call(TableChangeEvent {
                                                    pagination: pagination_total_for_change.map(|t| TablePaginationState {
                                                        total: t,
                                                        current: pagination_current_for_change.unwrap_or(1),
                                                        page_size: pagination_page_size_for_change.unwrap_or(10),
                                                    }),
                                                    sorter: Some(TableSorterState {
                                                        column_key: new_order.as_ref().map(|(k, _)| k.clone()),
                                                        order: new_order.as_ref().map(|(_, o)| *o),
                                                    }),
                                                    filters: HashMap::new(),
                                                });
                                            }
                                        }
                                    },
                                    span { class: "adui-table-column-title", "{title}" }
                                    if sortable {
                                        span { class: "adui-table-column-sorter",
                                            Icon { kind: IconKind::ArrowUp, size: 10.0 }
                                            Icon { kind: IconKind::ArrowDown, size: 10.0 }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // Table body
            div {
                class: "adui-table-body",
                style: "{scroll_style}",
                if loading {
                    Spin {
                        spinning: Some(true),
                        tip: Some("加载中...".to_string()),
                        div { class: "adui-table-body-inner",
                            {render_rows(&visible_columns, &data, &row_key_field, &row_class_name, has_selection, selection_type, &selected_keys, on_select_change)}
                        }
                    }
                } else if show_empty {
                    div { class: "adui-table-empty",
                        if let Some(node) = empty {
                            {node}
                        } else {
                            Empty {}
                        }
                    }
                } else {
                    div { class: "adui-table-body-inner",
                        {render_rows(&visible_columns, &data, &row_key_field, &row_class_name, has_selection, selection_type, &selected_keys, on_select_change)}
                    }
                }
            }

            if let Some(total) = pagination_total {
                div { class: "adui-table-pagination",
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
                            // Also emit to on_change
                            if let Some(cb) = on_change_cb {
                                cb.call(TableChangeEvent {
                                    pagination: Some(TablePaginationState {
                                        total,
                                        current: page,
                                        page_size: size,
                                    }),
                                    sorter: None,
                                    filters: HashMap::new(),
                                });
                            }
                        },
                    }
                }
            }
        }
    }
}

fn render_rows(
    columns: &[&TableColumn],
    data: &[Value],
    row_key_field: &Option<String>,
    row_class_name: &Option<String>,
    has_selection: bool,
    selection_type: SelectionType,
    selected_keys: &[String],
    on_select_change: Option<EventHandler<Vec<String>>>,
) -> Element {
    rsx! {
        {data.iter().enumerate().map(|(idx, row)| {
            let key = if let Some(field) = row_key_field {
                get_cell_text(row, field)
            } else {
                idx.to_string()
            };
            let row_class = row_class_name.clone().unwrap_or_default();
            let is_selected = selected_keys.contains(&key);
            let key_for_select = key.clone();
            let selected_keys_clone = selected_keys.to_vec();

            rsx! {
                div {
                    key: "{key}",
                    class: "adui-table-row {row_class}",
                    class: if is_selected { "adui-table-row-selected" } else { "" },
                    // Selection cell
                    if has_selection {
                        div { class: "adui-table-cell adui-table-cell-selection",
                            Checkbox {
                                checked: is_selected,
                                on_change: move |_checked| {
                                    if let Some(cb) = on_select_change {
                                        let mut new_keys = selected_keys_clone.clone();
                                        if is_selected {
                                            new_keys.retain(|k| k != &key_for_select);
                                        } else {
                                            if matches!(selection_type, SelectionType::Radio) {
                                                new_keys.clear();
                                            }
                                            new_keys.push(key_for_select.clone());
                                        }
                                        cb.call(new_keys);
                                    }
                                }
                            }
                        }
                    }
                    // Data cells
                    {columns.iter().map(|col| {
                        let mut cell_classes = vec!["adui-table-cell".to_string()];
                        if let Some(align) = col.align {
                            cell_classes.push(align.as_class().to_string());
                        }
                        if col.ellipsis {
                            cell_classes.push("adui-table-cell-ellipsis".into());
                        }
                        let cell_class = cell_classes.join(" ");

                        let data_index = col.data_index.as_ref().unwrap_or(&col.key);
                        let cell_value = row.get(data_index);

                        // Use custom render if provided, otherwise default text
                        let content = if let Some(render_fn) = col.render {
                            render_fn(cell_value, row, idx)
                        } else {
                            let text = get_cell_text(row, data_index);
                            rsx! { "{text}" }
                        };

                        rsx! {
                            div { class: "{cell_class}", {content} }
                        }
                    })}
                }
            }
        })}
    }
}

/// Helper to extract a cell value as string.
fn get_cell_text(row: &Value, key: &str) -> String {
    match row.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_builder_works() {
        let col = TableColumn::new("id", "ID")
            .width(100.0)
            .align(ColumnAlign::Center)
            .sortable();

        assert_eq!(col.key, "id");
        assert_eq!(col.title, "ID");
        assert_eq!(col.width, Some(100.0));
        assert_eq!(col.align, Some(ColumnAlign::Center));
        assert!(col.sortable);
    }

    #[test]
    fn sort_order_classes() {
        assert_eq!(
            SortOrder::Ascend.as_class(),
            "adui-table-column-sort-ascend"
        );
        assert_eq!(
            SortOrder::Descend.as_class(),
            "adui-table-column-sort-descend"
        );
    }
}
