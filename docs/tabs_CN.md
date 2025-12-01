# Tabs 标签页

## 概述

Tabs 组件将内容组织成多个可以切换的面板。它支持不同的视觉类型、位置和可编辑标签页。

## API 参考

### TabsProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<TabItem>` | - | 带 key/label/content 的标签项（必需） |
| `active_key` | `Option<String>` | `None` | 受控的活动键 |
| `default_active_key` | `Option<String>` | `None` | 非受控模式下的默认活动键 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 活动标签页改变时调用 |
| `type` | `TabsType` | `TabsType::Line` | 视觉类型 |
| `tab_placement` | `TabPlacement` | `TabPlacement::Top` | 标签页位置 |
| `centered` | `bool` | `false` | 是否居中标签页 |
| `hide_add` | `bool` | `false` | 隐藏添加按钮（用于 editable-card） |
| `on_edit` | `Option<EventHandler<TabEditAction>>` | `None` | 标签页添加/删除时调用 |
| `add_icon` | `Option<Element>` | `None` | 自定义添加图标 |
| `remove_icon` | `Option<Element>` | `None` | 自定义关闭图标 |
| `size` | `Option<ComponentSize>` | `None` | 视觉密度 |
| `destroy_inactive_tab_pane` | `bool` | `false` | 销毁非活动标签面板 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<TabsClassNames>` | `None` | 语义类名 |
| `styles` | `Option<TabsStyles>` | `None` | 语义样式 |

### TabItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 标签页的唯一键 |
| `label` | `String` | 标签页标签文本 |
| `disabled` | `bool` | 标签页是否禁用 |
| `closable` | `bool` | 标签页是否可以关闭（editable-card） |
| `icon` | `Option<Element>` | 自定义图标 |
| `content` | `Option<Element>` | 标签页内容 |

### TabsType

- `Line` - 线条样式标签页（默认）
- `Card` - 卡片样式标签页
- `EditableCard` - 可编辑卡片标签页，支持添加/删除

### TabPlacement

- `Top` - 标签页在顶部（默认）
- `Right` - 标签页在右侧
- `Bottom` - 标签页在底部
- `Left` - 标签页在左侧

## 使用示例

### 基础标签页

```rust
use adui_dioxus::{Tabs, TabItem};

rsx! {
    Tabs {
        items: vec![
            TabItem::new("1", "标签 1", Some(rsx!("内容 1"))),
            TabItem::new("2", "标签 2", Some(rsx!("内容 2"))),
            TabItem::new("3", "标签 3", Some(rsx!("内容 3"))),
        ],
    }
}
```

### 卡片标签页

```rust
use adui_dioxus::{Tabs, TabsType, TabItem};

rsx! {
    Tabs {
        r#type: TabsType::Card,
        items: vec![
            TabItem::new("1", "标签 1", Some(rsx!("内容 1"))),
            TabItem::new("2", "标签 2", Some(rsx!("内容 2"))),
        ],
    }
}
```

### 可编辑标签页

```rust
use adui_dioxus::{Tabs, TabsType, TabItem, TabEditAction};
use dioxus::prelude::*;

let tabs = use_signal(|| vec![
    TabItem::new("1", "标签 1", Some(rsx!("内容 1"))),
    TabItem::new("2", "标签 2", Some(rsx!("内容 2"))),
]);

rsx! {
    Tabs {
        r#type: TabsType::EditableCard,
        items: tabs.read().clone(),
        on_edit: Some(move |action| {
            match action {
                TabEditAction::Add => {
                    // 添加新标签页
                }
                TabEditAction::Remove(key) => {
                    // 删除标签页
                }
            }
        }),
    }
}
```

### 居中标签页

```rust
use adui_dioxus::{Tabs, TabItem};

rsx! {
    Tabs {
        centered: true,
        items: vec![
            TabItem::new("1", "标签 1", Some(rsx!("内容 1"))),
            TabItem::new("2", "标签 2", Some(rsx!("内容 2"))),
        ],
    }
}
```

## 使用场景

- **内容组织**：将内容组织成多个部分
- **设置面板**：在不同设置面板之间切换
- **数据视图**：在不同数据视图之间切换
- **文档**：组织文档部分

## 与 Ant Design 6.0.0 的差异

- ✅ 线条、卡片和可编辑卡片类型
- ✅ 多个位置
- ✅ 居中标签页
- ✅ 可编辑标签页，支持添加/删除
- ⚠️ 某些高级功能可能有所不同

