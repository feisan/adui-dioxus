# Drawer 抽屉

## 概述

Drawer 组件显示一个从屏幕边缘滑入的面板。通常用于设置、筛选器或附加内容。

## API 参考

### DrawerProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `open` | `bool` | - | 抽屉是否可见（必需） |
| `title` | `Option<String>` | `None` | 头部显示的可选标题 |
| `on_close` | `Option<EventHandler<()>>` | `None` | 抽屉关闭时调用 |
| `mask_closable` | `bool` | `true` | 点击遮罩是否关闭抽屉 |
| `closable` | `bool` | `true` | 显示关闭按钮 |
| `destroy_on_close` | `bool` | `false` | 关闭时销毁内容 |
| `placement` | `DrawerPlacement` | `DrawerPlacement::Right` | 抽屉位置 |
| `size` | `Option<f32>` | `None` | 逻辑尺寸（左右为宽度，上下为高度，默认为 378） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 抽屉内容（必需） |

### DrawerPlacement

- `Left` - 从左侧滑入
- `Right` - 从右侧滑入（默认）
- `Top` - 从顶部滑入
- `Bottom` - 从底部滑入

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Drawer, Button, ButtonType};
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    div {
        Button {
            onclick: move |_| {
                is_open.set(true);
            },
            "打开抽屉"
        }
        Drawer {
            open: *is_open.read(),
            title: Some("抽屉标题".to_string()),
            on_close: Some(move |_| {
                is_open.set(false);
            }),
            "抽屉内容在这里"
        }
    }
}
```

### 左侧位置

```rust
use adui_dioxus::{Drawer, DrawerPlacement};

rsx! {
    Drawer {
        open: true,
        placement: DrawerPlacement::Left,
        title: Some("左侧抽屉".to_string()),
        "从左侧的内容"
    }
}
```

### 自定义尺寸

```rust
use adui_dioxus::Drawer;

rsx! {
    Drawer {
        open: true,
        size: Some(500.0),
        title: Some("宽抽屉".to_string()),
        "宽内容"
    }
}
```

## 使用场景

- **设置面板**：显示设置
- **筛选器**：显示筛选选项
- **详情**：显示详细信息
- **导航**：侧边导航抽屉

## 与 Ant Design 6.0.0 的差异

- ✅ 基础抽屉功能
- ✅ 多个位置
- ✅ 自定义尺寸
- ✅ 遮罩和关闭按钮支持
- ⚠️ 某些高级功能可能有所不同
- ⚠️ 页脚支持尚未实现

