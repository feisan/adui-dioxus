# Divider 分割线

## 概述

Divider 组件用于通过水平或垂直线分隔内容区域。它可以包含可选的文本内容，并支持不同的方向和样式。

## API 参考

### DividerProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `dashed` | `bool` | `false` | 是否为虚线 |
| `plain` | `bool` | `false` | 是否为简洁样式（无边框） |
| `vertical` | `bool` | `false` | 是否为垂直分割线 |
| `orientation` | `DividerOrientation` | `DividerOrientation::Center` | 文本方向：`Left`、`Center`、`Right` |
| `orientation_margin` | `Option<String>` | `None` | 方向的边距（默认为 "16px"） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `content` | `Option<Element>` | `None` | 可选的内容/文本 |

### DividerOrientation

- `Left` - 文本左对齐
- `Center` - 文本居中（默认）
- `Right` - 文本右对齐

## 使用示例

### 基础水平分割线

```rust
use adui_dioxus::Divider;

rsx! {
    div {
        p { "上方内容" }
        Divider {}
        p { "下方内容" }
    }
}
```

### 带文本的分割线

```rust
use adui_dioxus::{Divider, DividerOrientation};

rsx! {
    Divider {
        content: Some(rsx!("或")),
        orientation: DividerOrientation::Center,
    }
}
```

### 虚线分割线

```rust
use adui_dioxus::Divider;

rsx! {
    Divider {
        dashed: true,
    }
}
```

### 垂直分割线

```rust
use adui_dioxus::Divider;

rsx! {
    div {
        style: "display: flex;",
        span { "左侧" }
        Divider { vertical: true }
        span { "右侧" }
    }
}
```

## 使用场景

- **内容分隔**：分隔不同的内容区域
- **表单分组**：将表单字段分为逻辑组
- **列表项**：分隔列表中的项目
- **垂直布局**：在水平布局中创建垂直分隔符

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础分割线样式
- ✅ 水平和垂直方向
- ✅ 带方向控制的文本内容
- ✅ 虚线和简洁变体
- ⚠️ 某些高级样式选项可能有所不同

