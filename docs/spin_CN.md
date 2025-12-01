# Spin 加载中

## 概述

Spin 组件显示加载中的旋转图标。它可以单独使用或包装内容，在激活时显示加载遮罩。

## API 参考

### SpinProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `spinning` | `Option<bool>` | `None`（默认为 `true`） | 旋转指示器是否激活 |
| `size` | `Option<SpinSize>` | `None`（默认为 `SpinSize::Default`） | 指示器的视觉尺寸 |
| `tip` | `Option<String>` | `None` | 指示器下方显示的可选文本 |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |
| `fullscreen` | `bool` | `false` | 是否作为全屏遮罩 |
| `children` | `Element` | - | 由旋转器包装的可选内容（必需） |

### SpinSize

- `Small` - 小尺寸旋转器
- `Default` - 默认尺寸旋转器
- `Large` - 大尺寸旋转器

## 使用示例

### 基础用法

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        div { "正在加载内容..." }
    }
}
```

### 带提示文本

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        tip: Some("加载中...".to_string()),
        div { "内容" }
    }
}
```

### 受控旋转状态

```rust
use adui_dioxus::Spin;
use dioxus::prelude::*;

let is_loading = use_signal(|| true);

rsx! {
    Spin {
        spinning: Some(*is_loading.read()),
        tip: Some("请稍候...".to_string()),
        div { "内容" }
    }
}
```

### 不同尺寸

```rust
use adui_dioxus::{Spin, SpinSize};

rsx! {
    div {
        Spin { size: Some(SpinSize::Small), div { "小" } }
        Spin { size: Some(SpinSize::Default), div { "默认" } }
        Spin { size: Some(SpinSize::Large), div { "大" } }
    }
}
```

### 全屏旋转器

```rust
use adui_dioxus::Spin;

rsx! {
    Spin {
        fullscreen: true,
        tip: Some("加载页面中...".to_string()),
        div { "页面内容" }
    }
}
```

## 使用场景

- **数据加载**：获取数据时显示旋转器
- **表单提交**：表单提交期间显示旋转器
- **内容加载**：包装正在加载的内容
- **全屏加载**：显示全屏加载状态

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础旋转器尺寸
- ✅ 提示文本支持
- ✅ 全屏模式
- ✅ 带遮罩的内容包装
- ⚠️ 某些高级样式选项可能有所不同
- ⚠️ 自定义指示器图标尚未实现

