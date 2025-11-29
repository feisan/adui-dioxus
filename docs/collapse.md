# Collapse 折叠面板

可以折叠/展开的内容区域。

## 何时使用

- 对复杂区域进行分组和隐藏，保持页面的整洁。
- 手风琴 是一种特殊的折叠面板，只允许单个内容区域展开。

## 代码演示

### 基础用法

最简单的用法。

```rust
use adui_dioxus::*;

let panels = vec![
    CollapsePanel::new(
        "1",
        rsx! { "面板标题 1" },
        rsx! { div { "面板内容 1" } },
    ),
    CollapsePanel::new(
        "2",
        rsx! { "面板标题 2" },
        rsx! { div { "面板内容 2" } },
    ),
];

rsx! {
    Collapse {
        items: panels,
        default_active_key: vec!["1".to_string()],
    }
}
```

### 手风琴模式

手风琴，每次只打开一个面板。

```rust
rsx! {
    Collapse {
        items: panels,
        accordion: true,
        default_active_key: vec!["1".to_string()],
    }
}
```

### 无边框

简洁的无边框样式。

```rust
rsx! {
    Collapse {
        items: panels,
        bordered: false,
    }
}
```

### 幽灵模式

透明背景，适合嵌入其他容器。

```rust
rsx! {
    Collapse {
        items: panels,
        ghost: true,
    }
}
```

### 自定义图标位置

可以将展开图标放在右侧。

```rust
rsx! {
    Collapse {
        items: panels,
        expand_icon_placement: ExpandIconPlacement::End,
    }
}
```

### 面板禁用

可以禁用某个面板。

```rust
let panels = vec![
    CollapsePanel::new("1", rsx! { "正常面板" }, rsx! { "内容" }),
    CollapsePanel::new("2", rsx! { "禁用面板" }, rsx! { "内容" })
        .disabled(true),
];
```

### 嵌套折叠面板

折叠面板可以嵌套使用。

```rust
let inner_panels = vec![/* ... */];

let outer_panels = vec![
    CollapsePanel::new(
        "outer",
        rsx! { "外层面板" },
        rsx! {
            div {
                "外层内容"
                Collapse { items: inner_panels }
            }
        },
    ),
];
```

## API

### Collapse Props

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| items | 面板数据 | `Vec<CollapsePanel>` | - |
| active_key | 当前激活的面板（受控） | `Option<Vec<String>>` | - |
| default_active_key | 初始化选中面板的 key（非受控） | `Option<Vec<String>>` | - |
| on_change | 切换面板的回调 | `Option<EventHandler<Vec<String>>>` | - |
| accordion | 手风琴模式 | `bool` | `false` |
| bordered | 是否有边框 | `bool` | `true` |
| ghost | 幽灵模式 | `bool` | `false` |
| size | 尺寸 | `Option<CollapseSize>` | `None` |
| expand_icon_placement | 图标位置 | `ExpandIconPlacement` | `Start` |
| collapsible | 所有面板的可折叠触发方式 | `Option<CollapsibleType>` | `None` |
| class | 自定义类名 | `Option<String>` | - |
| style | 自定义样式 | `Option<String>` | - |

### CollapsePanel

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| key | 唯一标识 | `String` | - |
| header | 面板头内容 | `Element` | - |
| content | 面板内容 | `Element` | - |
| disabled | 是否禁用 | `bool` | `false` |
| show_arrow | 是否显示箭头 | `bool` | `true` |
| collapsible | 可折叠触发方式 | `Option<CollapsibleType>` | `None` |
| extra | 额外内容 | `Option<Element>` | `None` |

### CollapseSize

```rust
pub enum CollapseSize {
    Small,
    Middle,  // 默认
    Large,
}
```

### ExpandIconPlacement

```rust
pub enum ExpandIconPlacement {
    Start,  // 默认，图标在左侧
    End,    // 图标在右侧
}
```

### CollapsibleType

```rust
pub enum CollapsibleType {
    Header,    // 点击头部触发（默认）
    Icon,      // 仅点击图标触发
    Disabled,  // 禁用
}
```

## 设计指引

- 折叠面板适用于需要对大量信息进行分组和隐藏的场景
- 手风琴模式适用于只需要展开一个面板的场景
- 无边框和幽灵模式适合嵌入其他容器中使用
- 可以根据需要自定义图标位置和触发方式

