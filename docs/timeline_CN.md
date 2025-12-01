# Timeline 时间轴

## 概述

Timeline 组件显示垂直或水平的事件时间轴。它支持不同的模式、方向和自定义图标。

## API 参考

### TimelineProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<TimelineItem>` | - | 要显示的时间轴项（必需） |
| `mode` | `TimelineMode` | `TimelineMode::Left` | 时间轴模式（位置） |
| `orientation` | `TimelineOrientation` | `TimelineOrientation::Vertical` | 时间轴方向 |
| `reverse` | `bool` | `false` | 是否反转项目顺序 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### TimelineItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 项的唯一键 |
| `title` | `Option<Element>` | 可选标题 |
| `content` | `Option<Element>` | 可选内容 |
| `icon` | `Option<Element>` | 可选自定义图标 |
| `color` | `Option<TimelineColor>` | 可选颜色预设 |
| `pending` | `bool` | 项是否处于待处理/加载状态 |

### TimelineMode

- `Left` - 所有项在左侧（默认）
- `Right` - 所有项在右侧
- `Alternate` - 项在左右之间交替

### TimelineOrientation

- `Vertical` - 垂直时间轴（默认）
- `Horizontal` - 水平时间轴

### TimelineColor

- `Blue` - 蓝色
- `Green` - 绿色
- `Red` - 红色
- `Gray` - 灰色

## 使用示例

### 基础时间轴

```rust
use adui_dioxus::{Timeline, TimelineItem};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1").title(rsx!("事件 1")).content(rsx!("描述 1")),
            TimelineItem::new("2").title(rsx!("事件 2")).content(rsx!("描述 2")),
            TimelineItem::new("3").title(rsx!("事件 3")).content(rsx!("描述 3")),
        ],
    }
}
```

### 交替模式

```rust
use adui_dioxus::{Timeline, TimelineMode, TimelineItem};

rsx! {
    Timeline {
        mode: TimelineMode::Alternate,
        items: vec![
            TimelineItem::new("1").title(rsx!("事件 1")),
            TimelineItem::new("2").title(rsx!("事件 2")),
            TimelineItem::new("3").title(rsx!("事件 3")),
        ],
    }
}
```

### 带颜色

```rust
use adui_dioxus::{Timeline, TimelineItem, TimelineColor};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1")
                .title(rsx!("成功"))
                .color(TimelineColor::Green),
            TimelineItem::new("2")
                .title(rsx!("错误"))
                .color(TimelineColor::Red),
        ],
    }
}
```

### 待处理项

```rust
use adui_dioxus::{Timeline, TimelineItem};

rsx! {
    Timeline {
        items: vec![
            TimelineItem::new("1").title(rsx!("已完成")),
            TimelineItem::new("2").title(rsx!("进行中")),
            TimelineItem::new("3").title(rsx!("待处理")).pending(true),
        ],
    }
}
```

## 使用场景

- **活动动态**：显示活动时间轴
- **流程跟踪**：跟踪流程步骤
- **历史记录**：显示历史事件
- **状态更新**：显示状态更新时间轴

## 与 Ant Design 6.0.0 的差异

- ✅ 垂直和水平方向
- ✅ 多种模式（左、右、交替）
- ✅ 自定义图标
- ✅ 颜色预设
- ✅ 待处理状态
- ⚠️ 某些高级功能可能有所不同

