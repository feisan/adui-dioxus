# Tree 树形控件

## 概述

Tree 组件以树形结构显示层次数据。它支持展开/折叠、选择、可勾选模式和键盘导航。

## API 参考

### TreeProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `tree_data` | `Option<Vec<TreeNode>>` | `None` | 树形数据源 |
| `expanded_keys` | `Option<Vec<String>>` | `None` | 受控的展开键 |
| `default_expanded_keys` | `Option<Vec<String>>` | `None` | 默认展开键 |
| `default_expand_all` | `bool` | `false` | 默认展开所有节点 |
| `auto_expand_parent` | `bool` | `true` | 自动展开父节点 |
| `on_expand` | `Option<EventHandler<Vec<String>>>` | `None` | 展开键改变时调用 |
| `selected_keys` | `Option<Vec<String>>` | `None` | 受控的选中键 |
| `default_selected_keys` | `Option<Vec<String>>` | `None` | 默认选中键 |
| `selectable` | `bool` | `true` | 节点是否可选择 |
| `multiple` | `bool` | `false` | 允许多选 |
| `on_select` | `Option<EventHandler<Vec<String>>>` | `None` | 选中键改变时调用 |
| `checkable` | `bool` | `false` | 启用复选框模式 |
| `checked_keys` | `Option<Vec<String>>` | `None` | 受控的勾选键 |
| `default_checked_keys` | `Option<Vec<String>>` | `None` | 默认勾选键 |
| `on_check` | `Option<EventHandler<Vec<String>>>` | `None` | 勾选键改变时调用 |
| `show_line` | `bool` | `false` | 显示连接节点的线条 |
| `block_node` | `bool` | `false` | 块节点样式 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### TreeNode

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 节点的唯一键 |
| `label` | `String` | 节点标签文本 |
| `disabled` | `bool` | 节点是否禁用 |
| `children` | `Vec<TreeNode>` | 子节点 |

## 使用示例

### 基础树形控件

```rust
use adui_dioxus::{Tree, TreeNode};

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "parent".to_string(),
                label: "父节点".to_string(),
                disabled: false,
                children: vec![
                    TreeNode {
                        key: "child1".to_string(),
                        label: "子节点 1".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                    TreeNode {
                        key: "child2".to_string(),
                        label: "子节点 2".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ]),
    }
}
```

### 可勾选树形控件

```rust
use adui_dioxus::{Tree, TreeNode};
use dioxus::prelude::*;

let checked = use_signal(|| vec![]);

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        checkable: true,
        checked_keys: Some(checked.read().clone()),
        on_check: Some(move |keys| {
            checked.set(keys);
        }),
    }
}
```

### 带选择

```rust
use adui_dioxus::{Tree, TreeNode};
use dioxus::prelude::*;

let selected = use_signal(|| vec![]);

rsx! {
    Tree {
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
        selected_keys: Some(selected.read().clone()),
        on_select: Some(move |keys| {
            selected.set(keys);
        }),
    }
}
```

### 显示线条

```rust
use adui_dioxus::{Tree, TreeNode};

rsx! {
    Tree {
        show_line: true,
        tree_data: Some(vec![
            TreeNode {
                key: "1".to_string(),
                label: "节点 1".to_string(),
                disabled: false,
                children: vec![],
            },
        ]),
    }
}
```

## 使用场景

- **文件浏览器**：显示文件系统结构
- **分类导航**：显示分类层次结构
- **组织架构图**：显示组织结构
- **菜单结构**：显示菜单层次结构

## 与 Ant Design 6.0.0 的差异

- ✅ 展开/折叠功能
- ✅ 单选和多选
- ✅ 可勾选模式
- ✅ 键盘导航
- ✅ 显示线条模式
- ⚠️ 拖放尚未实现
- ⚠️ 某些高级功能可能有所不同

