# Table：数据表格（MVP）

> 实现位置：`src/components/table.rs`
>
> 示例：`examples/table_demo.rs`

## 1. 设计目标

Table 用于展示结构化数据，是中后台应用的核心组件之一。

本版 Table 提供一个 **只读、轻量** 的 MVP 实现：

- 列定义由 `TableColumn` 描述；
- 行数据使用 `Vec<serde_json::Value>` 表示，列通过 key 访问对应字段；
- 支持基础边框、行布局与简单分页；
- 暂不实现排序/筛选/合并单元格等高级能力。

---

## 2. TableColumn / TableProps：组件属性

```rust
#[derive(Clone, PartialEq)]
pub struct TableColumn {
    pub key: String,
    pub title: String,
    pub width: Option<f32>,
    pub align: Option<ColumnAlign>,
    pub sortable: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    pub columns: Vec<TableColumn>,
    pub data: Vec<serde_json::Value>,
    pub row_key_field: Option<String>,
    pub row_class_name: Option<String>,
    pub bordered: bool,
    pub size: Option<ComponentSize>,
    pub loading: bool,
    pub is_empty: Option<bool>,
    pub empty: Option<Element>,
    pub pagination_total: Option<u32>,
    pub pagination_current: Option<u32>,
    pub pagination_page_size: Option<u32>,
    pub pagination_on_change: Option<EventHandler<(u32, u32)>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
}
```

字段说明：

- `TableColumn`：
  - `key`：对应数据中字段名；
  - `title`：表头标题文本；
  - `width`：可选列宽（像素）；
  - `align`：水平对齐方式（左/中/右），映射到 `.adui-table-align-*` 类；
  - `sortable`：预留字段，当前未启用排序逻辑；
- `columns`：
  - 列定义数组；
- `data`：
  - 行数据，使用 `serde_json::Value` 表示；
  - 通过 `row[ key ]` 访问单元格文本；
- `row_key_field`：
  - 可选用于作为行 key 的字段名；
  - 若为空，则使用行索引作为 key；
- `row_class_name`：
  - 行附加 class，用于高亮等自定义样式；
- `bordered` / `size`：
  - 外边框与尺寸（紧凑/默认/较大），对应 `.adui-table-bordered` 和 `.adui-table-sm/lg` 类；
- `loading` / `is_empty` / `empty`：
  - `loading = true`：使用 Spin 包裹表体，显示“加载中...”；
  - `is_empty`：由调用方根据数据判断是否为空；
  - `empty`：自定义空态 Element；若为空且 `is_empty = Some(true)`，使用内置 Empty；
- `pagination_*`：
  - 与 List 类似，控制分页组件：
    - `pagination_total`：总条数；
    - `pagination_current`：当前页；
    - `pagination_page_size`：每页条数；
    - `pagination_on_change`：页码变化时回调 `(page, page_size)`。

---

## 3. DOM 结构与样式类

典型结构：

```html
<div class="adui-table adui-table-bordered?">
  <div class="adui-table-header">
    <div class="adui-table-row adui-table-row-header">
      <div class="adui-table-cell adui-table-cell-header">姓名</div>
      <div class="adui-table-cell adui-table-cell-header">年龄</div>
      ...
    </div>
  </div>
  <div class="adui-table-body">
    <div class="adui-table-body-inner">
      <div class="adui-table-row">
        <div class="adui-table-cell">Alice</div>
        <div class="adui-table-cell">28</div>
        ...
      </div>
      ...
    </div>
  </div>
  <div class="adui-table-pagination"> <!-- 有分页时 -->
    <!-- Pagination -->
  </div>
</div>
```

主要类名：

- `.adui-table`：根容器；
- `.adui-table-bordered`：带边框样式；
- `.adui-table-header`：表头区域；
- `.adui-table-row`：行容器；
- `.adui-table-row-header`：表头行；
- `.adui-table-cell`：单元格；
- `.adui-table-align-left/center/right`：单元格文本对齐；
- `.adui-table-body-inner`：表体内部容器，支持简单斑马纹；
- `.adui-table-empty`：空态容器；
- `.adui-table-pagination`：分页区域容器。

> 样式定义集中在 `src/theme.rs` 的 `adui_table_style!` 宏，使用 ThemeTokens 控制背景、边框、间距与斑马纹效果。

---

## 4. 示例：基础表格

摘自 `examples/table_demo.rs`：

```rust
#[component]
fn TableDemoShell() -> Element {
    let columns = vec![
        TableColumn::new("name", "姓名"),
        TableColumn::new("age", "年龄"),
        TableColumn::new("city", "城市"),
    ];

    let data = vec![
        json!({ "name": "Alice", "age": 28, "city": "上海" }),
        json!({ "name": "Bob", "age": 35, "city": "北京" }),
        json!({ "name": "Charlie", "age": 42, "city": "深圳" }),
    ];

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Table demo" }
            p { "展示基础表格用法，包含简单列定义与数据映射。" }

            Table {
                columns: columns,
                data: data,
                bordered: true,
                size: Some(ComponentSize::Middle),
                loading: false,
                is_empty: Some(false),
            }
        }
    }
}
```

---

## 5. 与 Ant Design 的差异

相较于 AntD 6.x 的 Table，本版为大幅裁剪版：

- 未支持：
  - 排序（`sorter`）与排序图标；
  - 筛选（`filters`）、过滤下拉菜单；
  - 合并单元格、分组表头；
  - 虚拟滚动、固定列、树形数据、选择列等复杂能力；
- 数据模型：
  - 当前使用 `serde_json::Value` 作为行数据表示，便于快速迭代；
  - 后续可考虑引入泛型 `Table<RecordType>` 与回调型 `render`；
- 分页：
  - 仅支持基于 Pagination 的简单分页，不支持多位置分页条（顶部/底部同时）和 `showTotal` 等高级配置。

后续若需要，可以分阶段补充：

1. 排序与分页回调：在 `TableProps` 中增加排序状态与 `on_change` 回调；
2. 更丰富的数据模型：支持泛型记录类型与自定义单元格渲染；
3. 复杂布局：支持分组表头、固定列、横向滚动和行选择等特性。

此外，`data_view_demo` 展示了 Table 与 Layout/Breadcrumb/Pagination/Empty/Spin/Skeleton 的组合，而 `dashboard_demo` 则侧重展示 List + Tabs + Card + Tag + Badge + Avatar + Pagination 的仪表盘视图，两个示例可以结合参考。