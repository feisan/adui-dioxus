# Transfer 穿梭框

## 概述

Transfer 组件提供双列布局，用于在两个列表之间移动项目。它允许从源列表中选择项目并将其移动到目标列表。

## API 参考

### TransferProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `data_source` | `Vec<TransferItem>` | - | 穿梭框列表的数据源（必需） |
| `target_keys` | `Option<Vec<String>>` | `None` | 右侧（目标）列表中的项目键 |
| `selected_keys` | `Option<Vec<String>>` | `None` | 当前选中项目的键 |
| `titles` | `Option<(String, String)>` | `None` | 左右列表的标题 |
| `operations` | `Option<(String, String)>` | `None` | 穿梭操作按钮的文本 |
| `show_search` | `bool` | `false` | 是否显示搜索输入 |
| `search_placeholder` | `Option<String>` | `None` | 搜索输入的占位符文本 |
| `disabled` | `bool` | `false` | 组件是否禁用 |
| `show_select_all` | `bool` | `true` | 是否显示全选复选框 |
| `one_way` | `bool` | `false` | 单向模式（仅从左到右） |
| `on_change` | `Option<EventHandler<(Vec<String>, TransferDirection, Vec<String>)>>` | `None` | 目标键改变时调用 |
| `on_select_change` | `Option<EventHandler<(Vec<String>, Vec<String>)>>` | `None` | 选择改变时调用 |
| `on_search` | `Option<EventHandler<(TransferDirection, String)>>` | `None` | 搜索文本改变时调用 |
| `filter_option` | `Option<fn(&str, &TransferItem, TransferDirection) -> bool>` | `None` | 自定义过滤函数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### TransferItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 项目的唯一标识符 |
| `title` | `String` | 项目的显示标题 |
| `description` | `Option<String>` | 标题下方显示的可选描述 |
| `disabled` | `bool` | 此项目是否禁用 |

### TransferDirection

- `Left` - 左侧列表
- `Right` - 右侧列表

## 使用示例

### 基础穿梭框

```rust
use adui_dioxus::{Transfer, TransferItem};
use dioxus::prelude::*;

let target_keys = use_signal(|| vec![]);

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "项目 1"),
            TransferItem::new("2", "项目 2"),
            TransferItem::new("3", "项目 3"),
        ],
        target_keys: Some(target_keys.read().clone()),
        on_change: Some(move |(keys, _dir, _selected)| {
            target_keys.set(keys);
        }),
    }
}
```

### 带搜索

```rust
use adui_dioxus::{Transfer, TransferItem};

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "项目 1"),
            TransferItem::new("2", "项目 2"),
        ],
        show_search: true,
        search_placeholder: Some("搜索项目".to_string()),
    }
}
```

### 带自定义标题

```rust
use adui_dioxus::{Transfer, TransferItem};

rsx! {
    Transfer {
        data_source: vec![
            TransferItem::new("1", "项目 1"),
        ],
        titles: Some(("可用".to_string(), "已选".to_string())),
        operations: Some(("添加".to_string(), "移除".to_string())),
    }
}
```

## 使用场景

- **权限管理**：管理用户权限
- **角色分配**：为用户分配角色
- **数据选择**：从源到目标选择数据
- **列表管理**：在列表之间管理项目

## 与 Ant Design 6.0.0 的差异

- ✅ 双列布局
- ✅ 项目选择
- ✅ 搜索功能
- ✅ 单向模式
- ⚠️ 某些高级功能可能有所不同

