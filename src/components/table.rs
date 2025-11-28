use crate::components::config_provider::ComponentSize;
use crate::components::empty::Empty;
use crate::components::pagination::Pagination;
use crate::components::spin::Spin;
use dioxus::prelude::*;

/// Horizontal alignment for table cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnAlign {
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

/// Simple column definition for the Table component.
#[derive(Clone, PartialEq)]
pub struct TableColumn {
    pub key: String,
    pub title: String,
    pub width: Option<f32>,
    pub align: Option<ColumnAlign>,
    /// Whether this column can be sorted.
    pub sortable: bool,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            width: None,
            align: None,
            sortable: false,
        }
    }
}

/// Props for the Table component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    /// Column definitions. Each column corresponds to a header cell and a
    /// property in each row's data object.
    pub columns: Vec<TableColumn>,
    /// Row data represented as a vector of JSON values. This keeps the MVP
    /// implementation simple: columns read `row["data_index"]` when
    /// data_index == column.key.
    pub data: Vec<serde_json::Value>,
    /// Optional row key field (defaults to column key or index).
    #[props(optional)]
    pub row_key_field: Option<String>,
    /// Optional extra class applied to each row.
    #[props(optional)]
    pub row_class_name: Option<String>,
    /// Whether to show outer borders.
    #[props(default)]
    pub bordered: bool,
    /// Visual density.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Loading state; when true, the body is wrapped with Spin.
    #[props(default)]
    pub loading: bool,
    /// Whether the table is currently empty (used with `loading=false`).
    #[props(optional)]
    pub is_empty: Option<bool>,
    /// Custom empty node. When omitted and `is_empty = Some(true)`, the built
    /// in `Empty` component is used.
    #[props(optional)]
    pub empty: Option<Element>,
    /// Pagination total items.
    #[props(optional)]
    pub pagination_total: Option<u32>,
    /// Current page.
    #[props(optional)]
    pub pagination_current: Option<u32>,
    /// Page size.
    #[props(optional)]
    pub pagination_page_size: Option<u32>,
    /// Called when pagination changes.
    #[props(optional)]
    pub pagination_on_change: Option<EventHandler<(u32, u32)>>,
}

/// Ant Design flavored Table (MVP, readonly, no sorting yet).
#[component]
pub fn Table(props: TableProps) -> Element {
    let TableProps {
        columns,
        data,
        row_key_field,
        row_class_name,
        bordered,
        size,
        loading,
        is_empty,
        empty,
        pagination_total,
        pagination_current,
        pagination_page_size,
        pagination_on_change,
    } = props;

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
    let class_attr = class_list.join(" ");

    let show_empty = !loading && is_empty.unwrap_or(data.is_empty());

    // Helper to extract a cell value as string.
    fn get_cell_text(row: &serde_json::Value, key: &str) -> String {
        match row.get(key) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Number(n)) => n.to_string(),
            Some(serde_json::Value::Bool(b)) => b.to_string(),
            _ => "".to_string(),
        }
    }

    rsx! {
        div { class: "{class_attr}",
            // Table header
            div { class: "adui-table-header",
                div { class: "adui-table-row adui-table-row-header",
                    {columns.iter().map(|col| {
                        let mut cell_classes = vec!["adui-table-cell".to_string(), "adui-table-cell-header".to_string()];
                        if let Some(align) = col.align {
                            cell_classes.push(align.as_class().to_string());
                        }
                        let width_style = col.width.map(|w| format!("width: {}px;", w)).unwrap_or_default();
                        let title = col.title.clone();
                        let cell_class = cell_classes.join(" ");
                        rsx! {
                            div { class: "{cell_class}", style: "{width_style}", "{title}" }
                        }
                    })}
                }
            }

            // Table body
            if loading {
                div { class: "adui-table-body",
                    Spin {
                        spinning: Some(true),
                        tip: Some("加载中...".to_string()),
                        div { class: "adui-table-body-inner",
                            {data.iter().enumerate().map(|(idx, row)| {
                                let key = if let Some(field) = &row_key_field {
                                    get_cell_text(row, field)
                                } else {
                                    idx.to_string()
                                };
                                let row_class = row_class_name.clone().unwrap_or_default();
                                rsx! {
                                    div { key: "{key}", class: "adui-table-row {row_class}",
                                        {columns.iter().map(|col| {
                                            let mut cell_classes = vec!["adui-table-cell".to_string()];
                                            if let Some(align) = col.align {
                                                cell_classes.push(align.as_class().to_string());
                                            }
                                            let text = get_cell_text(row, &col.key);
                                            let cell_class = cell_classes.join(" ");
                                            rsx! {
                                                div { class: "{cell_class}", "{text}" }
                                            }
                                        })}
                                    }
                                }
                            })}
                        }
                    }
                }
            } else if show_empty {
                div { class: "adui-table-body",
                    div { class: "adui-table-empty",
                        if let Some(node) = empty {
                            {node}
                        } else {
                            Empty {}
                        }
                    }
                }
            } else {
                div { class: "adui-table-body",
                    div { class: "adui-table-body-inner",
                        {data.iter().enumerate().map(|(idx, row)| {
                            let key = if let Some(field) = &row_key_field {
                                get_cell_text(row, field)
                            } else {
                                idx.to_string()
                            };
                            let row_class = row_class_name.clone().unwrap_or_default();
                            rsx! {
                                div { key: "{key}", class: "adui-table-row {row_class}",
                                    {columns.iter().map(|col| {
                                        let mut cell_classes = vec!["adui-table-cell".to_string()];
                                        if let Some(align) = col.align {
                                            cell_classes.push(align.as_class().to_string());
                                        }
                                        let text = get_cell_text(row, &col.key);
                                        let cell_class = cell_classes.join(" ");
                                        rsx! {
                                            div { class: "{cell_class}", "{text}" }
                                        }
                                    })}
                                }
                            }
                        })}
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
                        },
                    }
                }
            }
        }
    }
}
