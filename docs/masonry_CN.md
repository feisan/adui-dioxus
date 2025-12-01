# Masonry 瀑布流

## 概述

Masonry 组件使用 CSS 列以瀑布流（Pinterest 风格）布局显示项目。它支持响应式列数和自定义间距。

## API 参考

### MasonryProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `columns` | `u16` | `3` | 列数 |
| `responsive` | `Option<MasonryResponsive>` | `None` | 响应式列配置 |
| `gap` | `Option<f32>` | `None` | 项目之间的间距（默认为 16px） |
| `row_gap` | `Option<f32>` | `None` | 行间距（默认为 gap 值） |
| `min_column_width` | `Option<f32>` | `None` | 最小列宽 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 瀑布流项目（必需） |

### MasonryResponsive

| 字段 | 类型 | 说明 |
|------|------|------|
| `xs` | `Option<u16>` | xs 屏幕的列数（< 576px） |
| `sm` | `Option<u16>` | sm 屏幕的列数（≥ 576px） |
| `md` | `Option<u16>` | md 屏幕的列数（≥ 768px） |
| `lg` | `Option<u16>` | lg 屏幕的列数（≥ 992px） |
| `xl` | `Option<u16>` | xl 屏幕的列数（≥ 1200px） |
| `xxl` | `Option<u16>` | xxl 屏幕的列数（≥ 1600px） |

## 使用示例

### 基础瀑布流

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        div { "项目 1" }
        div { "项目 2" }
        div { "项目 3" }
        div { "项目 4" }
    }
}
```

### 响应式瀑布流

```rust
use adui_dioxus::{Masonry, MasonryResponsive};

rsx! {
    Masonry {
        responsive: Some(MasonryResponsive {
            xs: Some(1),
            sm: Some(2),
            md: Some(3),
            lg: Some(4),
            xl: Some(5),
            xxl: Some(6),
        }),
        div { "项目 1" }
        div { "项目 2" }
        div { "项目 3" }
    }
}
```

### 自定义间距

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        gap: Some(24.0),
        row_gap: Some(16.0),
        div { "项目 1" }
        div { "项目 2" }
    }
}
```

### 带最小列宽

```rust
use adui_dioxus::Masonry;

rsx! {
    Masonry {
        columns: 3,
        min_column_width: Some(200.0),
        div { "项目 1" }
        div { "项目 2" }
    }
}
```

## 使用场景

- **图片画廊**：以瀑布流布局显示图片
- **卡片网格**：以 Pinterest 风格布局显示卡片
- **内容动态**：以瀑布流格式显示内容
- **作品集**：显示作品集项目

## 与 Ant Design 6.0.0 的差异

- ✅ 基于 CSS 列的瀑布流
- ✅ 响应式列数
- ✅ 自定义间距
- ✅ 最小列宽
- ⚠️ 某些高级功能可能有所不同

