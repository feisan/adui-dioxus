# Breadcrumb 面包屑

## 概述

Breadcrumb 组件显示导航路径，显示用户在层次结构中的当前位置。通常用于页面标题和导航区域。

## API 参考

### BreadcrumbProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<BreadcrumbItem>` | - | 有序的面包屑项目列表（必需） |
| `separator` | `Option<String>` | `None`（默认为 "/"） | 项目之间的分隔符字符串 |
| `class` | `Option<String>` | `None` | 应用于根的额外 CSS 类 |
| `style` | `Option<String>` | `None` | 应用于根的内联样式 |
| `on_item_click` | `Option<EventHandler<String>>` | `None` | 非最后一项的点击处理器（接收项目 id） |

### BreadcrumbItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `id` | `String` | 项目的唯一标识符 |
| `label` | `String` | 显示的文本标签 |
| `href` | `Option<String>` | 可选的链接目标 |

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::new("home", "首页"),
            BreadcrumbItem::new("category", "分类"),
            BreadcrumbItem::new("item", "当前项目"),
        ],
    }
}
```

### 带链接

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::with_href("home", "首页", "/"),
            BreadcrumbItem::with_href("category", "分类", "/category"),
            BreadcrumbItem::new("item", "当前项目"),
        ],
    }
}
```

### 自定义分隔符

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        separator: Some(">".to_string()),
        items: vec![
            BreadcrumbItem::new("1", "第一级"),
            BreadcrumbItem::new("2", "第二级"),
            BreadcrumbItem::new("3", "第三级"),
        ],
    }
}
```

### 带点击处理器

```rust
use adui_dioxus::{Breadcrumb, BreadcrumbItem};

rsx! {
    Breadcrumb {
        items: vec![
            BreadcrumbItem::new("home", "首页"),
            BreadcrumbItem::new("category", "分类"),
            BreadcrumbItem::new("item", "当前"),
        ],
        on_item_click: Some(move |id| {
            println!("点击了: {}", id);
        }),
    }
}
```

## 使用场景

- **页面导航**：显示当前页面位置
- **层次导航**：显示导航层次结构
- **面包屑路径**：帮助用户了解当前位置
- **深层导航**：在嵌套结构中导航

## 与 Ant Design 6.0.0 的差异

- ✅ 基础面包屑功能
- ✅ 自定义分隔符
- ✅ 链接支持
- ✅ 点击处理器
- ⚠️ 图标支持尚未实现
- ⚠️ 某些高级样式选项可能有所不同

