# Carousel 走马灯

## 概述

Carousel 组件提供轮播图，用于循环显示图片或内容。它支持多种过渡效果、自动播放和导航控件。

## API 参考

### CarouselProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<CarouselItem>` | `vec![]` | 要显示的幻灯片项目 |
| `slide_count` | `Option<usize>` | `None` | 幻灯片数量（使用 children 时） |
| `effect` | `CarouselEffect` | `CarouselEffect::ScrollX` | 过渡效果类型 |
| `autoplay` | `bool` | `false` | 是否启用自动播放 |
| `autoplay_speed` | `u64` | `3000` | 自动播放间隔（毫秒） |
| `dots` | `bool` | `true` | 是否显示导航点 |
| `dot_placement` | `DotPlacement` | `DotPlacement::Bottom` | 导航点的位置 |
| `arrows` | `bool` | `false` | 是否显示箭头导航 |
| `speed` | `u32` | `500` | 过渡速度（毫秒） |
| `initial_slide` | `usize` | `0` | 初始幻灯片索引 |
| `infinite` | `bool` | `true` | 启用无限循环 |
| `pause_on_hover` | `bool` | `true` | 悬停时暂停自动播放 |
| `on_change` | `Option<EventHandler<usize>>` | `None` | 幻灯片改变时调用 |
| `before_change` | `Option<EventHandler<(usize, usize)>>` | `None` | 幻灯片改变前调用 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 幻灯片内容（替代 items） |

### CarouselItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `content` | `String` | 要显示的内容 |
| `background` | `Option<String>` | 可选背景颜色 |

### CarouselEffect

- `ScrollX` - 水平滚动效果（默认）
- `Fade` - 淡入淡出效果

### DotPlacement

- `Top` - 顶部点
- `Bottom` - 底部点（默认）
- `Left` - 左侧点
- `Right` - 右侧点

## 使用示例

### 基础走马灯

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        items: vec![
            CarouselItem::new("幻灯片 1"),
            CarouselItem::new("幻灯片 2"),
            CarouselItem::new("幻灯片 3"),
        ],
    }
}
```

### 带自动播放

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        autoplay: true,
        autoplay_speed: 2000,
        items: vec![
            CarouselItem::new("幻灯片 1"),
            CarouselItem::new("幻灯片 2"),
        ],
    }
}
```

### 淡入淡出效果

```rust
use adui_dioxus::{Carousel, CarouselItem, CarouselEffect};

rsx! {
    Carousel {
        effect: CarouselEffect::Fade,
        items: vec![
            CarouselItem::new("幻灯片 1"),
            CarouselItem::new("幻灯片 2"),
        ],
    }
}
```

### 带箭头

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        arrows: true,
        items: vec![
            CarouselItem::new("幻灯片 1"),
            CarouselItem::new("幻灯片 2"),
        ],
    }
}
```

## 使用场景

- **图片画廊**：显示图片画廊
- **横幅**：显示促销横幅
- **内容轮播**：循环显示内容
- **产品展示**：展示产品

## 与 Ant Design 6.0.0 的差异

- ✅ 滚动和淡入淡出效果
- ✅ 自动播放支持
- ✅ 导航点
- ✅ 箭头导航
- ⚠️ 某些高级功能可能有所不同

