# List 列表

## 概述

List 组件显示项目列表，支持可选的头部、页脚、加载状态和分页。通常用于显示数据列表。

## API 参考

### ListProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `header` | `Option<Element>` | `None` | 可选的头部内容 |
| `footer` | `Option<Element>` | `None` | 可选的页脚内容 |
| `bordered` | `bool` | `false` | 是否渲染边框 |
| `size` | `Option<ComponentSize>` | `None` | 视觉密度 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `loading` | `bool` | `false` | 列表是否处于加载状态 |
| `is_empty` | `Option<bool>` | `None` | 数据集是否为空 |
| `empty` | `Option<Element>` | `None` | 自定义空内容 |
| `pagination_total` | `Option<u32>` | `None` | 分页总项目数 |
| `pagination_current` | `Option<u32>` | `None` | 当前页码（从1开始） |
| `pagination_page_size` | `Option<u32>` | `None` | 分页的页面大小 |
| `pagination_on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | 分页改变时的回调 |
| `children` | `Element` | - | 列表项内容（必需） |

## 使用示例

### 基础列表

```rust
use adui_dioxus::List;

rsx! {
    List {
        div { class: "adui-list-item", "项目 1" }
        div { class: "adui-list-item", "项目 2" }
        div { class: "adui-list-item", "项目 3" }
    }
}
```

### 带头部和页脚

```rust
use adui_dioxus::List;

rsx! {
    List {
        header: Some(rsx!("列表头部")),
        footer: Some(rsx!("列表页脚")),
        div { class: "adui-list-item", "项目 1" }
        div { class: "adui-list-item", "项目 2" }
    }
}
```

### 带边框列表

```rust
use adui_dioxus::List;

rsx! {
    List {
        bordered: true,
        div { class: "adui-list-item", "项目 1" }
        div { class: "adui-list-item", "项目 2" }
    }
}
```

### 加载状态

```rust
use adui_dioxus::List;

rsx! {
    List {
        loading: true,
        div { class: "adui-list-item", "项目 1" }
        div { class: "adui-list-item", "项目 2" }
    }
}
```

### 带分页

```rust
use adui_dioxus::List;
use dioxus::prelude::*;

let current_page = use_signal(|| 1u32);

rsx! {
    List {
        pagination_total: Some(100),
        pagination_current: Some(*current_page.read()),
        pagination_on_change: Some(move |(page, _size)| {
            current_page.set(page);
        }),
        div { class: "adui-list-item", "项目 1" }
        div { class: "adui-list-item", "项目 2" }
    }
}
```

## 使用场景

- **数据列表**：显示数据项目列表
- **用户列表**：显示用户列表
- **产品列表**：显示产品目录
- **活动动态**：显示活动动态

## 与 Ant Design 6.0.0 的差异

- ✅ 基础列表功能
- ✅ 头部和页脚
- ✅ 加载状态
- ✅ 空状态
- ✅ 分页支持
- ⚠️ 网格布局尚未实现
- ⚠️ 某些高级功能可能有所不同

