# Collapse 折叠面板

## 概述

Collapse 组件显示可折叠的内容面板。它支持手风琴模式、自定义图标和各种样式选项。

## API 参考

### CollapseProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<CollapsePanel>` | - | 要显示的面板项（必需） |
| `active_key` | `Option<Vec<String>>` | `None` | 受控的活动键（展开的面板） |
| `default_active_key` | `Option<Vec<String>>` | `None` | 非受控模式下的默认活动键 |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | 活动键改变时调用 |
| `accordion` | `bool` | `false` | 手风琴模式（只有一个面板展开） |
| `bordered` | `bool` | `true` | 是否显示边框 |
| `ghost` | `bool` | `false` | 幽灵模式（透明背景） |
| `size` | `Option<CollapseSize>` | `None` | 尺寸变体 |
| `expand_icon_placement` | `ExpandIconPlacement` | `ExpandIconPlacement::Start` | 展开图标位置 |
| `collapsible` | `Option<CollapsibleType>` | `None` | 所有面板的默认可折叠类型 |
| `destroy_on_hidden` | `bool` | `true` | 销毁非活动面板内容 |
| `expand_icon` | `Option<ExpandIconRenderFn>` | `None` | 自定义展开图标渲染函数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<CollapseClassNames>` | `None` | 语义类名 |
| `styles` | `Option<CollapseStyles>` | `None` | 语义样式 |

### CollapsePanel

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 面板的唯一键 |
| `header` | `Element` | 面板头部内容 |
| `content` | `Element` | 面板内容 |
| `disabled` | `bool` | 面板是否禁用 |
| `show_arrow` | `bool` | 是否显示展开箭头 |
| `collapsible` | `Option<CollapsibleType>` | 此面板的可折叠类型 |
| `extra` | `Option<Element>` | 头部中的额外内容 |

### CollapseSize

- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

### CollapsibleType

- `Header` - 点击头部触发
- `Icon` - 仅点击图标触发
- `Disabled` - 禁用，无法触发

### ExpandIconPlacement

- `Start` - 图标在开始（默认）
- `End` - 图标在结束

## 使用示例

### 基础折叠面板

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        items: vec![
            CollapsePanel::new("1", rsx!("面板 1"), rsx!("内容 1")),
            CollapsePanel::new("2", rsx!("面板 2"), rsx!("内容 2")),
            CollapsePanel::new("3", rsx!("面板 3"), rsx!("内容 3")),
        ],
    }
}
```

### 手风琴模式

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        accordion: true,
        items: vec![
            CollapsePanel::new("1", rsx!("面板 1"), rsx!("内容 1")),
            CollapsePanel::new("2", rsx!("面板 2"), rsx!("内容 2")),
        ],
    }
}
```

### 幽灵模式

```rust
use adui_dioxus::{Collapse, CollapsePanel};

rsx! {
    Collapse {
        ghost: true,
        items: vec![
            CollapsePanel::new("1", rsx!("面板 1"), rsx!("内容 1")),
        ],
    }
}
```

### 带额外内容

```rust
use adui_dioxus::{Collapse, CollapsePanel, Button};

rsx! {
    Collapse {
        items: vec![
            CollapsePanel::new("1", rsx!("面板 1"), rsx!("内容 1"))
                .extra(rsx!(Button { "操作" })),
        ],
    }
}
```

## 使用场景

- **常见问题部分**：显示常见问题
- **设置面板**：将设置组织成可折叠部分
- **内容组织**：将内容组织成可折叠部分
- **手风琴**：创建手风琴式界面

## 与 Ant Design 6.0.0 的差异

- ✅ 基础折叠功能
- ✅ 手风琴模式
- ✅ 幽灵模式
- ✅ 自定义图标
- ✅ 尺寸变体
- ⚠️ 某些高级功能可能有所不同

