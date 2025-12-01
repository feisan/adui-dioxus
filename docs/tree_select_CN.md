# TreeSelect 树选择器

## 概述

TreeSelect 组件提供树形结构的选择界面。它结合了树视图和选择下拉框的功能，允许用户从分层结构中选择项目。

## API 参考

### TreeSelectProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `tree_data` | `Option<Vec<TreeNode>>` | `None` | 树数据源 |
| `value` | `Option<String>` | `None` | 单选的受控值 |
| `values` | `Option<Vec<String>>` | `None` | 多选的受控值 |
| `multiple` | `bool` | `false` | 启用多选 |
| `tree_checkable` | `bool` | `false` | 在树节点中显示复选框 |
| `show_search` | `bool` | `false` | 启用搜索功能 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `size` | `Option<ComponentSize>` | `None` | 组件尺寸 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `dropdown_class` | `Option<String>` | `None` | 下拉的额外类 |
| `dropdown_style` | `Option<String>` | `None` | 下拉的额外样式 |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | 选择改变时调用 |

### TreeNode

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 节点的唯一键 |
| `label` | `String` | 显示标签 |
| `disabled` | `bool` | 节点是否禁用 |
| `children` | `Vec<TreeNode>` | 子节点 |

## 使用示例

### 基础树选择器

```rust
use adui_dioxus::{TreeSelect, TreeNode};
use dioxus::prelude::*;

let value = use_signal(|| None::<String>);

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![
                    TreeNode {
                        key: "1-1".to_string(),
                        label: "节点 1-1".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ]),
        value: value.read().clone(),
        on_change: Some(move |values| {
            value.set(values.first().cloned());
        }),
        placeholder: Some("选择节点".to_string()),
    }
}
```

### 多选

```rust
use adui_dioxus::{TreeSelect, TreeNode};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        multiple: true,
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
        placeholder: Some("选择节点".to_string()),
    }
}
```

### 带复选框

```rust
use adui_dioxus::{TreeSelect, TreeNode};

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        tree_checkable: true,
        placeholder: Some("选择节点".to_string()),
    }
}
```

### 带搜索

```rust
use adui_dioxus::{TreeSelect, TreeNode};

rsx! {
    TreeSelect {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        show_search: true,
        placeholder: Some("搜索并选择".to_string()),
    }
}
```

## 使用场景

- **分层选择**：从分层结构中选择项目
- **类别选择**：在树结构中选择类别
- **组织选择**：选择组织单位
- **多级选择**：从多级数据中选择

## 与 Ant Design 6.0.0 的差异

- ✅ 单选和多选
- ✅ 树结构
- ✅ 搜索功能
- ✅ 复选框模式
- ⚠️ 某些高级功能可能有所不同

