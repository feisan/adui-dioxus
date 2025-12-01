# Pagination 分页

## 概述

Pagination 组件允许用户浏览内容页面。它支持页面大小选择、总数显示和受控/非受控模式。

## API 参考

### PaginationProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `current` | `Option<u32>` | `None` | 受控的当前页码（从1开始） |
| `default_current` | `Option<u32>` | `None` | 非受控时的初始页码 |
| `page_size` | `Option<u32>` | `None` | 受控的页面大小 |
| `default_page_size` | `Option<u32>` | `None` | 非受控时的初始页面大小（默认为10） |
| `total` | `u32` | - | 项目总数（必需） |
| `page_size_options` | `Option<Vec<u32>>` | `None` | 页面大小选择器的可选选项 |
| `show_size_changer` | `bool` | `false` | 是否显示页面大小选择下拉框 |
| `show_total` | `bool` | `false` | 是否显示总数文本 |
| `on_change` | `Option<EventHandler<(u32, u32)>>` | `None` | 页码或页面大小改变时调用（页码，页面大小） |
| `on_page_size_change` | `Option<EventHandler<(u32, u32)>>` | `None` | 页面大小改变时调用（页码，页面大小） |
| `class` | `Option<String>` | `None` | 根的额外类名 |
| `style` | `Option<String>` | `None` | 根的内联样式 |

## 使用示例

### 基础用法

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 100,
        default_current: Some(1),
        default_page_size: Some(10),
    }
}
```

### 受控分页

```rust
use adui_dioxus::Pagination;
use dioxus::prelude::*;

let current_page = use_signal(|| 1u32);
let page_size = use_signal(|| 10u32);

rsx! {
    Pagination {
        current: Some(*current_page.read()),
        page_size: Some(*page_size.read()),
        total: 100,
        on_change: Some(move |(page, size)| {
            current_page.set(page);
            page_size.set(size);
        }),
    }
}
```

### 带页面大小选择器

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 200,
        show_size_changer: true,
        page_size_options: Some(vec![10, 20, 50, 100]),
        default_page_size: Some(20),
    }
}
```

### 显示总数

```rust
use adui_dioxus::Pagination;

rsx! {
    Pagination {
        total: 150,
        show_total: true,
        default_current: Some(1),
    }
}
```

## 使用场景

- **数据表格**：对表格数据进行分页
- **列表**：浏览列表页面
- **搜索结果**：对搜索结果进行分页
- **内容页面**：浏览内容页面

## 与 Ant Design 6.0.0 的差异

- ✅ 基础分页功能
- ✅ 页面大小选择器
- ✅ 总数显示
- ✅ 受控和非受控模式
- ⚠️ 简单分页模式尚未实现
- ⚠️ 某些高级样式选项可能有所不同

