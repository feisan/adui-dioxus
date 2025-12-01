# FloatButton 悬浮按钮

## 概述

FloatButton 组件显示一个固定在页面上的悬浮操作按钮。它支持主要和默认类型、危险样式，并且可以分组。

## API 参考

### FloatButtonProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `type` | `FloatButtonType` | `FloatButtonType::Primary` | 视觉类型 |
| `shape` | `FloatButtonShape` | `FloatButtonShape::Circle` | 按钮形状 |
| `danger` | `bool` | `false` | 按钮是否使用危险样式 |
| `href` | `Option<String>` | `None` | 如果提供，将渲染为链接 |
| `icon` | `Option<Element>` | `None` | 图标元素 |
| `description` | `Option<String>` | `None` | 悬停时显示的描述文本 |
| `content` | `Option<String>` | `None` | 文本内容（图标的替代） |
| `tooltip` | `Option<String>` | `None` | 工具提示文本 |
| `right` | `Option<f32>` | `None` | 右侧位置（默认为 24px） |
| `left` | `Option<f32>` | `None` | 左侧位置 |
| `top` | `Option<f32>` | `None` | 顶部位置 |
| `bottom` | `Option<f32>` | `None` | 底部位置（默认为 72px） |
| `z_index` | `Option<i32>` | `None` | Z-index（默认为 99） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | 点击事件处理器 |
| `children` | `Element` | - | 按钮内容（必需） |

### FloatButtonGroupProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `shape` | `FloatButtonShape` | `FloatButtonShape::Circle` | 所有按钮的共享形状 |
| `type` | `FloatButtonType` | `FloatButtonType::Primary` | 所有按钮的共享类型 |
| `gap` | `f32` | `12.0` | 按钮之间的间距 |
| `right` | `Option<f32>` | `None` | 右侧位置 |
| `left` | `Option<f32>` | `None` | 左侧位置 |
| `top` | `Option<f32>` | `None` | 顶部位置 |
| `bottom` | `Option<f32>` | `None` | 底部位置 |
| `z_index` | `Option<i32>` | `None` | Z-index |
| `pure` | `bool` | `false` | 纯面板模式（无定位） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 悬浮按钮子元素（必需） |

### BackTopProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `visibility_height` | `Option<f32>` | `None` | 显示按钮的滚动高度 |
| `right` | `Option<f32>` | `None` | 右侧位置 |
| `bottom` | `Option<f32>` | `None` | 底部位置 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 按钮内容（必需） |

### FloatButtonType

- `Default` - 默认类型
- `Primary` - 主要类型（默认）

### FloatButtonShape

- `Circle` - 圆形按钮（默认）
- `Square` - 方形按钮

## 使用示例

### 基础悬浮按钮

```rust
use adui_dioxus::{FloatButton, Icon, IconKind};

rsx! {
    FloatButton {
        icon: Some(rsx!(Icon { kind: IconKind::Plus, size: 20.0 })),
    }
}
```

### 带描述

```rust
use adui_dioxus::{FloatButton, Icon, IconKind};

rsx! {
    FloatButton {
        icon: Some(rsx!(Icon { kind: IconKind::Edit, size: 20.0 })),
        description: Some("编辑".to_string()),
    }
}
```

### 悬浮按钮组

```rust
use adui_dioxus::{FloatButton, FloatButtonGroup, Icon, IconKind};

rsx! {
    FloatButtonGroup {
        FloatButton {
            icon: Some(rsx!(Icon { kind: IconKind::Plus, size: 20.0 })),
        }
        FloatButton {
            icon: Some(rsx!(Icon { kind: IconKind::Edit, size: 20.0 })),
        }
    }
}
```

### 回到顶部

```rust
use adui_dioxus::BackTop;

rsx! {
    BackTop {
        visibility_height: Some(400.0),
    }
}
```

## 使用场景

- **主要操作**：快速访问主要操作
- **回到顶部**：滚动到顶部功能
- **操作组**：多个悬浮操作
- **快速访问**：快速访问常用操作

## 与 Ant Design 6.0.0 的差异

- ✅ 基础悬浮按钮功能
- ✅ 按钮组
- ✅ BackTop 组件
- ✅ 自定义定位
- ⚠️ 拖拽功能尚未实现
- ⚠️ 某些高级功能可能有所不同

