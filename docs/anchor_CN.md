# Anchor 锚点

## 概述

Anchor 组件提供页面内滚动的导航链接。它根据滚动位置自动高亮活动部分。

## API 参考

### AnchorProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<AnchorLinkItem>` | - | 锚点链接项列表（必需） |
| `affix` | `bool` | `true` | 滚动时是否使用 Affix 固定锚点 |
| `offset_top` | `Option<f64>` | `None` | 固定时的顶部偏移量（像素） |
| `bounds` | `f64` | `5.0` | 计算活动状态时的边界距离 |
| `target_offset` | `Option<f64>` | `None` | 点击锚点时的滚动偏移量 |
| `direction` | `AnchorDirection` | `AnchorDirection::Vertical` | 锚点导航方向 |
| `replace` | `bool` | `false` | 是否替换当前历史记录条目 |
| `show_ink_in_fixed` | `bool` | `false` | 在固定模式下显示墨水指示器 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 活动链接改变时调用 |
| `on_click` | `Option<EventHandler<AnchorClickInfo>>` | `None` | 锚点链接被点击时调用 |
| `get_current_anchor` | `Option<fn(String) -> String>` | `None` | 确定活动锚点的自定义函数 |
| `class` | `Option<String>` | `None` | 额外的 CSS 类 |
| `style` | `Option<String>` | `None` | 额外的内联样式 |

### AnchorLinkItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 项的唯一键 |
| `href` | `String` | 目标锚点 href（例如 "#section-1"） |
| `title` | `String` | 链接的显示标题 |
| `target` | `Option<String>` | 可选的目标属性 |
| `children` | `Option<Vec<AnchorLinkItem>>` | 嵌套子项（仅垂直） |

### AnchorDirection

- `Vertical` - 垂直锚点导航（默认）
- `Horizontal` - 水平锚点导航

## 使用示例

### 基础锚点

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "章节 1"),
            AnchorLinkItem::new("2", "#section-2", "章节 2"),
            AnchorLinkItem::new("3", "#section-3", "章节 3"),
        ],
    }
}
```

### 带嵌套项

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        items: vec![
            AnchorLinkItem::with_children(
                "1",
                "#section-1",
                "章节 1",
                vec![
                    AnchorLinkItem::new("1-1", "#section-1-1", "子章节 1.1"),
                    AnchorLinkItem::new("1-2", "#section-1-2", "子章节 1.2"),
                ],
            ),
        ],
    }
}
```

### 自定义偏移量

```rust
use adui_dioxus::{Anchor, AnchorLinkItem};

rsx! {
    Anchor {
        offset_top: Some(100.0),
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "章节 1"),
        ],
    }
}
```

### 水平锚点

```rust
use adui_dioxus::{Anchor, AnchorDirection, AnchorLinkItem};

rsx! {
    Anchor {
        direction: AnchorDirection::Horizontal,
        items: vec![
            AnchorLinkItem::new("1", "#section-1", "章节 1"),
            AnchorLinkItem::new("2", "#section-2", "章节 2"),
        ],
    }
}
```

## 使用场景

- **长页面**：在长页面中导航
- **文档**：文档的目录
- **单页应用**：在单页应用中导航
- **章节**：跳转到页面的不同章节

## 与 Ant Design 6.0.0 的差异

- ✅ 垂直和水平方向
- ✅ 嵌套锚点链接
- ✅ 基于滚动的自动高亮
- ✅ Affix 支持
- ⚠️ 某些高级功能可能有所不同

