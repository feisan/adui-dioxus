# Timeline 时间轴

垂直展示的时间流信息。

## 何时使用

- 当有一系列信息需按时间排列时；
- 需要有一条时间轴进行视觉上的串联时。

## 代码演示

### 基础用法

基本的时间轴。

```rust
use adui_dioxus::*;

let items = vec![
    TimelineItem::new("1")
        .title(rsx! { Text { "创建服务" } })
        .content(rsx! { Text { "2024-11-29 09:00:00" } })
        .color(TimelineColor::Green),
    TimelineItem::new("2")
        .title(rsx! { Text { "初步审核" } })
        .content(rsx! { Text { "2024-11-29 10:30:00" } })
        .color(TimelineColor::Green),
    TimelineItem::new("3")
        .title(rsx! { Text { "最终审批" } })
        .content(rsx! { Text { "等待处理..." } })
        .color(TimelineColor::Gray),
];

rsx! {
    Timeline {
        items: items,
    }
}
```

### 不同颜色

使用不同颜色标识不同状态。

```rust
let items = vec![
    TimelineItem::new("1")
        .title(rsx! { Text { "成功" } })
        .color(TimelineColor::Green),
    TimelineItem::new("2")
        .title(rsx! { Text { "进行中" } })
        .color(TimelineColor::Blue),
    TimelineItem::new("3")
        .title(rsx! { Text { "错误" } })
        .color(TimelineColor::Red),
    TimelineItem::new("4")
        .title(rsx! { Text { "默认" } })
        .color(TimelineColor::Gray),
];
```

### 自定义图标

可以自定义节点图标。

```rust
let items = vec![
    TimelineItem::new("1")
        .title(rsx! { Text { "已完成" } })
        .icon(rsx! { Icon { kind: IconKind::Check } }),
    TimelineItem::new("2")
        .title(rsx! { Text { "进行中" } })
        .icon(rsx! { Icon { kind: IconKind::Loading, spin: true } }),
    TimelineItem::new("3")
        .title(rsx! { Text { "失败" } })
        .icon(rsx! { Icon { kind: IconKind::Close } }),
];
```

### 进行中状态

最后一个时间节点为进行中状态。

```rust
let items = vec![
    TimelineItem::new("1")
        .title(rsx! { Text { "步骤一" } })
        .content(rsx! { Text { "已完成" } })
        .color(TimelineColor::Green),
    TimelineItem::new("2")
        .title(rsx! { Text { "步骤二" } })
        .content(rsx! { Text { "已完成" } })
        .color(TimelineColor::Green),
    TimelineItem::new("3")
        .title(rsx! { Text { "步骤三" } })
        .content(rsx! { Text { "正在处理中..." } })
        .pending(true),  // 显示 loading 图标
];
```

### 交替展示

内容在时间轴两侧轮流出现。

```rust
rsx! {
    Timeline {
        items: items,
        mode: TimelineMode::Alternate,
    }
}
```

### 右侧时间轴

时间轴和内容在右侧。

```rust
rsx! {
    Timeline {
        items: items,
        mode: TimelineMode::Right,
    }
}
```

### 倒序排列

倒序展示时间轴。

```rust
rsx! {
    Timeline {
        items: items,
        reverse: true,  // 反转顺序
    }
}
```

## API

### Timeline Props

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| items | 时间轴项数据 | `Vec<TimelineItem>` | - |
| mode | 时间轴位置模式 | `TimelineMode` | `Left` |
| orientation | 方向 | `TimelineOrientation` | `Vertical` |
| reverse | 是否倒序 | `bool` | `false` |
| class | 自定义类名 | `Option<String>` | - |
| style | 自定义样式 | `Option<String>` | - |

### TimelineItem

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| key | 唯一标识 | `String` | - |
| title | 标题 | `Option<Element>` | `None` |
| content | 内容 | `Option<Element>` | `None` |
| icon | 自定义图标 | `Option<Element>` | `None` |
| color | 圆圈颜色 | `Option<TimelineColor>` | `None` |
| pending | 是否为进行中状态 | `bool` | `false` |

### TimelineMode

```rust
pub enum TimelineMode {
    Left,       // 默认，左侧时间轴
    Right,      // 右侧时间轴
    Alternate,  // 交替展示
}
```

### TimelineOrientation

```rust
pub enum TimelineOrientation {
    Vertical,    // 默认，垂直方向
    Horizontal,  // 水平方向（暂未实现完整支持）
}
```

### TimelineColor

```rust
pub enum TimelineColor {
    Blue,   // 蓝色
    Green,  // 绿色
    Red,    // 红色
    Gray,   // 灰色
}
```

## 设计指引

- 时间轴用于展示时间流信息，适合操作日志、发展历程等场景
- 使用不同颜色可以区分不同状态（成功、进行中、失败等）
- 可以自定义图标以提供更丰富的视觉表达
- pending 状态会自动显示 loading 图标，适合表示进行中的节点
- 交替模式适合内容较多的场景，能更好地利用空间
- 倒序排列适合显示最新记录在前的场景

