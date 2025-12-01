# Skeleton 骨架屏

## 概述

Skeleton 组件显示带有动画骨架块的占位加载状态。用于在内容准备好之前显示加载占位符。

## API 参考

### SkeletonProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `loading` | `Option<bool>` | `None`（默认为 `true`） | 是否显示骨架屏（为 false 时渲染内容） |
| `active` | `bool` | `false` | 是否显示活动动画 |
| `title` | `bool` | `true` | 是否渲染标题块 |
| `paragraph_rows` | `Option<u8>` | `None`（默认为 3） | 段落行数 |
| `class` | `Option<String>` | `None` | 根的额外类名 |
| `style` | `Option<String>` | `None` | 根的内联样式 |
| `content` | `Option<Element>` | `None` | 包装的内容（当 loading 为 false 时渲染） |

## 使用示例

### 基础骨架屏

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {}
}
```

### 自定义行数

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        paragraph_rows: Some(5),
    }
}
```

### 活动动画

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        active: true,
    }
}
```

### 无标题

```rust
use adui_dioxus::Skeleton;

rsx! {
    Skeleton {
        title: false,
        paragraph_rows: Some(2),
    }
}
```

### 带内容

```rust
use adui_dioxus::Skeleton;
use dioxus::prelude::*;

let is_loading = use_signal(|| true);

rsx! {
    Skeleton {
        loading: Some(*is_loading.read()),
        content: Some(rsx! {
            div { "实际内容在这里" }
        }),
    }
}
```

## 使用场景

- **内容加载**：在内容加载时显示骨架屏
- **卡片加载**：在卡片中显示骨架屏
- **列表加载**：为列表项显示骨架屏
- **页面加载**：为整个页面显示骨架屏

## 与 Ant Design 6.0.0 的差异

- ✅ 基础骨架块
- ✅ 标题和段落行
- ✅ 活动动画
- ✅ 内容包装
- ⚠️ 头像骨架屏尚未实现
- ⚠️ 按钮骨架屏尚未实现
- ⚠️ 某些高级样式选项可能有所不同

