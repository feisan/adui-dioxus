# Layout 布局

## 概述

Layout 组件提供页面布局容器，包含 Header、Footer、Sider 和 Content 部分。通常用于在应用程序中构建页面布局。

## API 参考

### LayoutProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `has_sider` | `Option<bool>` | `None` | 布局是否包含 Sider（如果为 None 则自动检测） |
| `children` | `Element` | - | 布局子元素（必需） |

### SiderProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `width` | `Option<f32>` | `None` | 侧边栏宽度（像素） |
| `collapsed_width` | `Option<f32>` | `None` | 折叠时的宽度 |
| `collapsed` | `Option<bool>` | `None` | 受控的折叠状态 |
| `default_collapsed` | `bool` | `false` | 初始折叠状态 |
| `collapsible` | `bool` | `false` | 侧边栏是否可以折叠 |
| `reverse_arrow` | `bool` | `false` | 反转箭头方向 |
| `trigger` | `Option<Element>` | `None` | 自定义触发器元素 |
| `zero_width_trigger_style` | `Option<String>` | `None` | 零宽度触发器的样式 |
| `theme` | `SiderTheme` | `SiderTheme::Dark` | 侧边栏主题 |
| `has_border` | `bool` | `true` | 是否显示边框 |
| `on_collapse` | `Option<EventHandler<bool>>` | `None` | 折叠状态改变时调用 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 侧边栏内容（必需） |

### SiderTheme

- `Light` - 浅色主题
- `Dark` - 深色主题（默认）

## 使用示例

### 基础布局

```rust
use adui_dioxus::{Layout, Header, Content, Footer};

rsx! {
    Layout {
        Header {
            "头部"
        }
        Content {
            "内容"
        }
        Footer {
            "页脚"
        }
    }
}
```

### 带侧边栏

```rust
use adui_dioxus::{Layout, Header, Sider, Content, Footer};

rsx! {
    Layout {
        has_sider: Some(true),
        Sider {
            width: Some(200.0),
            "侧边栏"
        }
        Layout {
            Header { "头部" }
            Content { "内容" }
            Footer { "页脚" }
        }
    }
}
```

### 可折叠侧边栏

```rust
use adui_dioxus::{Layout, Sider, Content};
use dioxus::prelude::*;

let collapsed = use_signal(|| false);

rsx! {
    Layout {
        Sider {
            collapsed: Some(*collapsed.read()),
            collapsible: true,
            on_collapse: Some(move |is_collapsed| {
                collapsed.set(is_collapsed);
            }),
            "侧边栏内容"
        }
        Content {
            "主要内容"
        }
    }
}
```

## 使用场景

- **页面结构**：构建页面布局
- **管理仪表板**：创建管理仪表板布局
- **侧边导航**：为页面添加侧边导航
- **响应式布局**：创建响应式页面布局

## 与 Ant Design 6.0.0 的差异

- ✅ 基础布局组件（Header、Footer、Content、Sider）
- ✅ 可折叠侧边栏
- ✅ 侧边栏主题
- ⚠️ 某些高级功能可能有所不同
- ⚠️ 响应式断点可能有所不同

