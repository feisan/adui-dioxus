# Table 表格

## 概述

Table 组件以结构化表格格式显示数据，支持排序、过滤、分页、行选择和可展开行。

## API 参考

### TableProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `columns` | `Vec<TableColumn>` | - | 列定义（必需） |
| `data` | `Vec<Value>` | - | 行数据（JSON 值）（必需） |
| `row_key_field` | `Option<String>` | `None` | 用作行键的字段 |
| `row_class_name` | `Option<String>` | `None` | 每行的额外类 |
| `row_class_name_fn` | `Option<RowClassNameFn>` | `None` | 动态行类名函数 |
| `row_props_fn` | `Option<RowPropsFn>` | `None` | 动态行属性函数 |
| `bordered` | `bool` | `false` | 是否显示外边框 |
| `size` | `Option<ComponentSize>` | `None` | 视觉密度 |
| `loading` | `bool` | `false` | 加载状态 |
| `is_empty` | `Option<bool>` | `None` | 表格是否为空 |
| `empty` | `Option<Element>` | `None` | 自定义空节点 |
| `row_selection` | `Option<RowSelection>` | `None` | 行选择配置 |
| `scroll` | `Option<TableScroll>` | `None` | 滚动配置 |
| `sticky` | `Option<StickyConfig>` | `None` | 粘性头部配置 |
| `expandable` | `Option<ExpandableConfig>` | `None` | 可展开行配置 |
| `summary` | `Option<SummaryConfig>` | `None` | 汇总行配置 |
| `on_change` | `Option<EventHandler<TableChangeEvent>>` | `None` | 分页/过滤/排序改变时调用 |
| `get_popup_container` | `Option<String>` | `None` | 弹出窗口的容器选择器 |
| `virtual` | `bool` | `false` | 启用虚拟滚动 |
| `locale` | `Option<TableLocale>` | `None` | 语言环境配置 |
| `show_header` | `bool` | `true` | 显示表格头部 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<TableClassNames>` | `None` | 语义类名 |
| `styles` | `Option<TableStyles>` | `None` | 语义样式 |
| `pagination_total` | `Option<u32>` | `None` | 分页总项目数 |
| `pagination_current` | `Option<u32>` | `None` | 当前页码 |
| `pagination_page_size` | `Option<u32>` | `None` | 页面大小 |
| `pagination_on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | 分页改变回调 |

### TableColumn

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 唯一列键 |
| `title` | `String` | 列标题 |
| `data_index` | `Option<String>` | 数据字段索引 |
| `width` | `Option<f32>` | 列宽 |
| `align` | `Option<ColumnAlign>` | 单元格对齐 |
| `fixed` | `Option<ColumnFixed>` | 固定位置 |
| `sortable` | `bool` | 列是否可排序 |
| `default_sort_order` | `Option<SortOrder>` | 默认排序顺序 |
| `sorter` | `Option<ColumnSorterFn>` | 自定义排序函数 |
| `filters` | `Option<Vec<ColumnFilter>>` | 过滤选项 |
| `on_filter` | `Option<ColumnFilterFn>` | 自定义过滤函数 |
| `render` | `Option<ColumnRenderFn>` | 自定义渲染函数 |
| `hidden` | `bool` | 列是否隐藏 |
| `ellipsis` | `bool` | 文本溢出省略 |

### ColumnAlign

- `Left` - 左对齐（默认）
- `Center` - 居中对齐
- `Right` - 右对齐

### SortOrder

- `Ascend` - 升序
- `Descend` - 降序

### ColumnFixed

- `Left` - 固定在左侧
- `Right` - 固定在右侧

## 使用示例

### 基础表格

```rust
use adui_dioxus::{Table, TableColumn};
use serde_json::json;

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "姓名"),
            TableColumn::new("age", "年龄"),
            TableColumn::new("email", "邮箱"),
        ],
        data: vec![
            json!({"name": "张三", "age": 30, "email": "zhang@example.com"}),
            json!({"name": "李四", "age": 25, "email": "li@example.com"}),
        ],
    }
}
```

### 带排序

```rust
use adui_dioxus::{Table, TableColumn, SortOrder};

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "姓名")
                .sortable(),
            TableColumn::new("age", "年龄")
                .sortable()
                .default_sort_order(Some(SortOrder::Ascend)),
        ],
        data: vec![],
    }
}
```

### 带行选择

```rust
use adui_dioxus::{Table, TableColumn, RowSelection, SelectionType};
use dioxus::prelude::*;
use serde_json::json;

let selected = use_signal(|| vec![]);

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "姓名"),
        ],
        data: vec![
            json!({"name": "张三"}),
        ],
        row_selection: Some(RowSelection {
            selected_row_keys: selected.read().clone(),
            on_change: Some(move |keys| {
                selected.set(keys);
            }),
            selection_type: SelectionType::Checkbox,
            preserve_selected_row_keys: false,
        }),
    }
}
```

### 带分页

```rust
use adui_dioxus::{Table, TableColumn};
use dioxus::prelude::*;
use serde_json::json;

let current_page = use_signal(|| 1u32);

rsx! {
    Table {
        columns: vec![
            TableColumn::new("name", "姓名"),
        ],
        data: vec![
            json!({"name": "张三"}),
        ],
        pagination_total: Some(100),
        pagination_current: Some(*current_page.read()),
        pagination_on_change: Some(move |(page, _size)| {
            current_page.set(page);
        }),
    }
}
```

## 使用场景

- **数据展示**：显示结构化数据
- **数据管理**：通过排序和过滤管理数据
- **报表**：生成数据报表
- **仪表板**：显示仪表板数据

## 与 Ant Design 6.0.0 的差异

- ✅ 排序和过滤
- ✅ 行选择
- ✅ 分页
- ✅ 可展开行
- ✅ 固定列
- ⚠️ 某些高级功能可能有所不同

