# Menu 导航菜单

## 概述

Menu 组件提供水平和垂直模式的导航菜单。它支持嵌套菜单项、选择和展开控制。

## API 参考

### MenuProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<MenuItemNode>` | - | 树形结构的菜单项（必需） |
| `mode` | `MenuMode` | `MenuMode::Inline` | 显示模式 |
| `selected_keys` | `Option<Vec<String>>` | `None` | 受控的选中键 |
| `default_selected_keys` | `Option<Vec<String>>` | `None` | 默认选中键 |
| `open_keys` | `Option<Vec<String>>` | `None` | 受控的展开键（内联模式） |
| `default_open_keys` | `Option<Vec<String>>` | `None` | 默认展开键 |
| `on_select` | `Option<EventHandler<String>>` | `None` | 菜单项被选中时调用 |
| `on_open_change` | `Option<EventHandler<Vec<String>>>` | `None` | 展开键改变时调用 |
| `inline_collapsed` | `bool` | `false` | 内联菜单是否折叠 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### MenuItemNode

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 项的唯一标识符 |
| `label` | `String` | 显示标签 |
| `icon` | `Option<Element>` | 可选图标 |
| `disabled` | `bool` | 项是否禁用 |
| `children` | `Option<Vec<MenuItemNode>>` | 嵌套子项（最多两级） |

### MenuMode

- `Inline` - 内联模式（默认，用于侧边栏）
- `Horizontal` - 水平模式（用于头部）

## 使用示例

### 基础菜单

```rust
use adui_dioxus::{Menu, MenuItemNode};

rsx! {
    Menu {
        items: vec![
            MenuItemNode::leaf("1", "项目 1"),
            MenuItemNode::leaf("2", "项目 2"),
            MenuItemNode::leaf("3", "项目 3"),
        ],
    }
}
```

### 水平菜单

```rust
use adui_dioxus::{Menu, MenuMode, MenuItemNode};

rsx! {
    Menu {
        mode: MenuMode::Horizontal,
        items: vec![
            MenuItemNode::leaf("1", "首页"),
            MenuItemNode::leaf("2", "关于"),
            MenuItemNode::leaf("3", "联系"),
        ],
    }
}
```

### 带嵌套项

```rust
use adui_dioxus::{Menu, MenuItemNode};

rsx! {
    Menu {
        items: vec![
            MenuItemNode {
                id: "1".to_string(),
                label: "父级".to_string(),
                icon: None,
                disabled: false,
                children: Some(vec![
                    MenuItemNode::leaf("1-1", "子项 1"),
                    MenuItemNode::leaf("1-2", "子项 2"),
                ]),
            },
        ],
    }
}
```

### 带选择

```rust
use adui_dioxus::{Menu, MenuItemNode};
use dioxus::prelude::*;

let selected = use_signal(|| vec!["1".to_string()]);

rsx! {
    Menu {
        items: vec![
            MenuItemNode::leaf("1", "项目 1"),
            MenuItemNode::leaf("2", "项目 2"),
        ],
        selected_keys: Some(selected.read().clone()),
        on_select: Some(move |key| {
            selected.set(vec![key]);
        }),
    }
}
```

## 使用场景

- **导航**：创建导航菜单
- **侧边栏**：构建侧边栏导航
- **头部**：创建头部导航
- **设置**：组织设置菜单

## 与 Ant Design 6.0.0 的差异

- ✅ 内联和水平模式
- ✅ 嵌套菜单项（最多两级）
- ✅ 选择控制
- ✅ 展开控制
- ⚠️ 某些高级功能可能有所不同

