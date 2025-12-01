# Tag 标签

## 概述

Tag 组件用于标记和分类内容。它支持预设颜色、可关闭功能和可选状态。

## API 参考

### TagProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `color` | `Option<TagColor>` | `None` | 标签的预设颜色 |
| `closable` | `bool` | `false` | 标签是否可以关闭 |
| `on_close` | `Option<EventHandler<()>>` | `None` | 关闭图标被点击时的回调 |
| `checkable` | `bool` | `false` | 标签是否可选（可切换选择） |
| `checked` | `Option<bool>` | `None` | 可选标签的受控选中状态 |
| `default_checked` | `Option<bool>` | `None` | 非受控可选标签的默认选中状态 |
| `on_change` | `Option<EventHandler<bool>>` | `None` | 选中状态改变时的回调 |
| `class` | `Option<String>` | `None` | 标签的额外类名 |
| `style` | `Option<String>` | `None` | 标签的内联样式 |
| `children` | `Element` | - | 标签内容（必需） |

### TagColor

- `Default` - 默认颜色
- `Primary` - 主要颜色
- `Success` - 成功颜色（绿色）
- `Warning` - 警告颜色（橙色）
- `Error` - 错误颜色（红色）

## 使用示例

### 基础用法

```rust
use adui_dioxus::Tag;

rsx! {
    Tag {
        "标签文本"
    }
}
```

### 带颜色

```rust
use adui_dioxus::{Tag, TagColor};

rsx! {
    div {
        Tag { color: Some(TagColor::Default), "默认" }
        Tag { color: Some(TagColor::Primary), "主要" }
        Tag { color: Some(TagColor::Success), "成功" }
        Tag { color: Some(TagColor::Warning), "警告" }
        Tag { color: Some(TagColor::Error), "错误" }
    }
}
```

### 可关闭标签

```rust
use adui_dioxus::Tag;

rsx! {
    Tag {
        closable: true,
        on_close: Some(move |_| {
            println!("标签已关闭");
        }),
        "可关闭标签"
    }
}
```

### 可选标签

```rust
use adui_dioxus::Tag;
use dioxus::prelude::*;

let checked = use_signal(|| false);

rsx! {
    Tag {
        checkable: true,
        checked: Some(*checked.read()),
        on_change: Some(move |is_checked| {
            checked.set(is_checked);
        }),
        "可选标签"
    }
}
```

## 使用场景

- **标签**：分类和标记内容
- **筛选器**：使用可选标签进行筛选
- **状态指示器**：用彩色标签显示状态
- **可移除项**：使用可关闭标签表示可移除项
- **标签输入**：构建标签输入组件

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有预设颜色
- ✅ 可关闭功能
- ✅ 可选状态
- ✅ 受控和非受控模式
- ⚠️ 自定义颜色（hex/rgb）尚未实现
- ⚠️ 某些高级样式选项可能有所不同

