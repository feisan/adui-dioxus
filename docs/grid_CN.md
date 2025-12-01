# Grid 栅格

## 概述

Grid 组件提供 24 列栅格系统用于响应式布局。它由 `Row` 和 `Col` 组件组成，共同创建灵活、响应式的布局。

## API 参考

### RowProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `gutter` | `Option<f32>` | `None` | 水平间距 |
| `gutter_vertical` | `Option<f32>` | `None` | 垂直间距 |
| `responsive_gutter` | `Option<ResponsiveGutter>` | `None` | 响应式间距配置 |
| `gutter_spec` | `Option<RowGutter>` | `None` | 间距规格（统一/配对/响应式） |
| `justify` | `RowJustify` | `RowJustify::Start` | 水平对齐 |
| `align` | `RowAlign` | `RowAlign::Top` | 交叉轴对齐 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 行的子元素（Col 组件）（必需） |

### ColProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `span` | `Option<f32>` | `None` | 跨越的列数（1-24） |
| `offset` | `Option<f32>` | `None` | 偏移的列数 |
| `order` | `Option<f32>` | `None` | 显示顺序 |
| `push` | `Option<f32>` | `None` | 向右推的列数 |
| `pull` | `Option<f32>` | `None` | 向左拉的列数 |
| `flex` | `Option<String>` | `None` | Flex 值 |
| `xs` | `Option<f32>` | `None` | xs 断点的跨度 |
| `sm` | `Option<f32>` | `None` | sm 断点的跨度 |
| `md` | `Option<f32>` | `None` | md 断点的跨度 |
| `lg` | `Option<f32>` | `None` | lg 断点的跨度 |
| `xl` | `Option<f32>` | `None` | xl 断点的跨度 |
| `xxl` | `Option<f32>` | `None` | xxl 断点的跨度 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 列内容（必需） |

### RowJustify

- `Start` - 起始对齐（默认）
- `End` - 结束对齐
- `Center` - 居中对齐
- `SpaceAround` - 环绕间距
- `SpaceBetween` - 两端对齐
- `SpaceEvenly` - 均匀间距

### RowAlign

- `Top` - 顶部对齐（默认）
- `Middle` - 中间对齐
- `Bottom` - 底部对齐
- `Stretch` - 拉伸对齐

## 使用示例

### 基础栅格

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col { span: Some(8.0), "列 1" }
        Col { span: Some(8.0), "列 2" }
        Col { span: Some(8.0), "列 3" }
    }
}
```

### 带间距

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        gutter: Some(16.0),
        Col { span: Some(12.0), "列 1" }
        Col { span: Some(12.0), "列 2" }
    }
}
```

### 响应式栅格

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col {
            xs: Some(24.0),
            sm: Some(12.0),
            md: Some(8.0),
            lg: Some(6.0),
            "响应式列"
        }
    }
}
```

### 带偏移

```rust
use adui_dioxus::{Row, Col};

rsx! {
    Row {
        Col { span: Some(8.0), "列 1" }
        Col { span: Some(8.0), offset: Some(8.0), "列 2" }
    }
}
```

## 使用场景

- **页面布局**：创建响应式页面布局
- **表单布局**：组织表单字段
- **内容网格**：在网格中显示内容
- **仪表板**：创建仪表板布局

## 与 Ant Design 6.0.0 的差异

- ✅ 24 列栅格系统
- ✅ 响应式断点
- ✅ 间距
- ✅ Flex 支持
- ⚠️ 某些高级功能可能有所不同

