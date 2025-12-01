# Flex 弹性布局

## 概述

Flex 组件提供弹性盒子布局容器，支持可配置的对齐、换行和间距。它支持水平和垂直方向。

## API 参考

### FlexProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `direction` | `FlexDirection` | `FlexDirection::Row` | Flex 方向 |
| `justify` | `FlexJustify` | `FlexJustify::Start` | 主轴对齐 |
| `align` | `FlexAlign` | `FlexAlign::Stretch` | 交叉轴对齐 |
| `wrap` | `FlexWrap` | `FlexWrap::NoWrap` | 换行行为 |
| `orientation` | `Option<FlexOrientation>` | `None` | 方向辅助 |
| `vertical` | `bool` | `false` | 垂直布局 |
| `component` | `FlexComponent` | `FlexComponent::Div` | 根元素类型 |
| `gap` | `Option<f32>` | `None` | 项目之间的间距 |
| `row_gap` | `Option<f32>` | `None` | 行间距 |
| `column_gap` | `Option<f32>` | `None` | 列间距 |
| `gap_size` | `Option<FlexGap>` | `None` | 预设间距大小 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | Flex 子元素（必需） |

### FlexDirection

- `Row` - 行方向（默认）
- `RowReverse` - 行反向
- `Column` - 列方向
- `ColumnReverse` - 列反向

### FlexJustify

- `Start` - 起始对齐（默认）
- `End` - 结束对齐
- `Center` - 居中对齐
- `Between` - 两端对齐
- `Around` - 环绕对齐
- `Evenly` - 均匀对齐

### FlexAlign

- `Start` - 起始对齐
- `End` - 结束对齐
- `Center` - 居中对齐
- `Stretch` - 拉伸对齐（默认）
- `Baseline` - 基线对齐

### FlexWrap

- `NoWrap` - 不换行（默认）
- `Wrap` - 换行
- `WrapReverse` - 反向换行

### FlexGap

- `Small` - 小间距
- `Middle` - 中等间距
- `Large` - 大间距

## 使用示例

### 基础 Flex

```rust
use adui_dioxus::Flex;

rsx! {
    Flex {
        div { "项目 1" }
        div { "项目 2" }
        div { "项目 3" }
    }
}
```

### 垂直 Flex

```rust
use adui_dioxus::Flex;

rsx! {
    Flex {
        vertical: true,
        div { "项目 1" }
        div { "项目 2" }
    }
}
```

### 带间距

```rust
use adui_dioxus::{Flex, FlexGap};

rsx! {
    Flex {
        gap_size: Some(FlexGap::Middle),
        div { "项目 1" }
        div { "项目 2" }
    }
}
```

### 带对齐

```rust
use adui_dioxus::{Flex, FlexJustify, FlexAlign};

rsx! {
    Flex {
        justify: FlexJustify::Center,
        align: FlexAlign::Center,
        div { "居中项目" }
    }
}
```

### 带换行

```rust
use adui_dioxus::{Flex, FlexWrap};

rsx! {
    Flex {
        wrap: FlexWrap::Wrap,
        div { "项目 1" }
        div { "项目 2" }
        div { "项目 3" }
    }
}
```

## 使用场景

- **布局**：创建弹性布局
- **组件排列**：带间距排列组件
- **响应式设计**：创建响应式布局
- **对齐**：在容器中对齐项目

## 与 Ant Design 6.0.0 的差异

- ✅ Flex 方向和换行
- ✅ 对齐选项
- ✅ 间距
- ✅ 预设间距大小
- ⚠️ 某些高级功能可能有所不同

