# Space 间距

## 概述

Space 组件在子元素之间提供间距。它支持水平和垂直布局、自定义间距、对齐和可选分隔符。

## API 参考

### SpaceProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `direction` | `SpaceDirection` | `SpaceDirection::Horizontal` | 布局方向 |
| `size` | `SpaceSize` | `SpaceSize::Middle` | 间距的预设尺寸 |
| `gap` | `Option<f32>` | `None` | 自定义间距值（覆盖 size） |
| `gap_cross` | `Option<f32>` | `None` | 交叉轴间距值 |
| `wrap` | `Option<bool>` | `None` | 是否换行（根据方向默认） |
| `align` | `SpaceAlign` | `SpaceAlign::Start` | 交叉轴对齐 |
| `compact` | `bool` | `false` | 紧凑模式（减少间距） |
| `split` | `Option<Element>` | `None` | 项目之间的可选分隔符元素 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 子元素（必需） |

### SpaceDirection

- `Horizontal` - 水平布局（默认）
- `Vertical` - 垂直布局

### SpaceAlign

- `Start` - 对齐到开始（默认）
- `End` - 对齐到结束
- `Center` - 居中对齐
- `Baseline` - 基线对齐

### SpaceSize

- `Small` - 小间距
- `Middle` - 中等间距（默认）
- `Large` - 大间距

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Space, Button, ButtonType};

rsx! {
    Space {
        Button { r#type: ButtonType::Primary, "按钮 1" }
        Button { r#type: ButtonType::Default, "按钮 2" }
        Button { r#type: ButtonType::Dashed, "按钮 3" }
    }
}
```

### 垂直布局

```rust
use adui_dioxus::{Space, SpaceDirection, Button, ButtonType};

rsx! {
    Space {
        direction: SpaceDirection::Vertical,
        Button { r#type: ButtonType::Primary, "顶部" }
        Button { r#type: ButtonType::Default, "中间" }
        Button { r#type: ButtonType::Dashed, "底部" }
    }
}
```

### 自定义间距

```rust
use adui_dioxus::Space;

rsx! {
    Space {
        gap: Some(32.0),
        div { "项目 1" }
        div { "项目 2" }
        div { "项目 3" }
    }
}
```

### 带分隔符

```rust
use adui_dioxus::{Space, Divider};

rsx! {
    Space {
        split: Some(rsx!(Divider { vertical: true })),
        span { "项目 1" }
        span { "项目 2" }
        span { "项目 3" }
    }
}
```

### 紧凑模式

```rust
use adui_dioxus::Space;

rsx! {
    Space {
        compact: true,
        Button { "紧凑 1" }
        Button { "紧凑 2" }
        Button { "紧凑 3" }
    }
}
```

## 使用场景

- **按钮组**：均匀分布按钮
- **表单字段**：在表单元素之间添加间距
- **列表**：为列表项添加间距
- **灵活布局**：在布局中创建灵活的间距

## 与 Ant Design 6.0.0 的差异

- ✅ 水平和垂直布局
- ✅ 预设和自定义间距尺寸
- ✅ 对齐选项
- ✅ 分隔符支持
- ✅ 紧凑模式
- ⚠️ 某些高级样式选项可能有所不同

