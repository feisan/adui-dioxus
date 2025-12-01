# Carousel

## Overview

The Carousel component provides a slideshow for cycling through images or content. It supports various transition effects, autoplay, and navigation controls.

## API Reference

### CarouselProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `items` | `Vec<CarouselItem>` | `vec![]` | Slide items to display |
| `slide_count` | `Option<usize>` | `None` | Number of slides (when using children) |
| `effect` | `CarouselEffect` | `CarouselEffect::ScrollX` | Transition effect type |
| `autoplay` | `bool` | `false` | Whether to enable autoplay |
| `autoplay_speed` | `u64` | `3000` | Autoplay interval in milliseconds |
| `dots` | `bool` | `true` | Whether to show navigation dots |
| `dot_placement` | `DotPlacement` | `DotPlacement::Bottom` | Position of navigation dots |
| `arrows` | `bool` | `false` | Whether to show arrow navigation |
| `speed` | `u32` | `500` | Transition speed in milliseconds |
| `initial_slide` | `usize` | `0` | Initial slide index |
| `infinite` | `bool` | `true` | Enable infinite loop |
| `pause_on_hover` | `bool` | `true` | Pause autoplay on hover |
| `on_change` | `Option<EventHandler<usize>>` | `None` | Called when slide changes |
| `before_change` | `Option<EventHandler<(usize, usize)>>` | `None` | Called before slide changes |
| `class` | `Option<String>` | `None` | Extra class name |
| `style` | `Option<String>` | `None` | Inline style |
| `children` | `Element` | - | Slide content (alternative to items) |

### CarouselItem

| Field | Type | Description |
|-------|------|-------------|
| `content` | `String` | Content to display |
| `background` | `Option<String>` | Optional background color |

### CarouselEffect

- `ScrollX` - Horizontal scrolling effect (default)
- `Fade` - Fade in/out effect

### DotPlacement

- `Top` - Dots at the top
- `Bottom` - Dots at the bottom (default)
- `Left` - Dots at the left
- `Right` - Dots at the right

## Usage Examples

### Basic Carousel

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        items: vec![
            CarouselItem::new("Slide 1"),
            CarouselItem::new("Slide 2"),
            CarouselItem::new("Slide 3"),
        ],
    }
}
```

### With Autoplay

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        autoplay: true,
        autoplay_speed: 2000,
        items: vec![
            CarouselItem::new("Slide 1"),
            CarouselItem::new("Slide 2"),
        ],
    }
}
```

### Fade Effect

```rust
use adui_dioxus::{Carousel, CarouselItem, CarouselEffect};

rsx! {
    Carousel {
        effect: CarouselEffect::Fade,
        items: vec![
            CarouselItem::new("Slide 1"),
            CarouselItem::new("Slide 2"),
        ],
    }
}
```

### With Arrows

```rust
use adui_dioxus::{Carousel, CarouselItem};

rsx! {
    Carousel {
        arrows: true,
        items: vec![
            CarouselItem::new("Slide 1"),
            CarouselItem::new("Slide 2"),
        ],
    }
}
```

## Use Cases

- **Image Galleries**: Display image galleries
- **Banners**: Show promotional banners
- **Content Slideshows**: Cycle through content
- **Product Showcases**: Showcase products

## Differences from Ant Design 6.0.0

- ✅ Scroll and fade effects
- ✅ Autoplay support
- ✅ Navigation dots
- ✅ Arrow navigation
- ⚠️ Some advanced features may differ

