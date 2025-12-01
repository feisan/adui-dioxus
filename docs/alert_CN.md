# Alert 警告提示

## 概述

Alert 组件用于向用户显示重要消息，如警告、错误、成功消息或信息通知。它支持多种类型，具有不同的视觉样式，并且可以关闭。

## API 参考

### AlertProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `type` | `AlertType` | `AlertType::Info` | 警告类型：`Success`、`Info`、`Warning`、`Error` |
| `message` | `Element` | - | 主要消息内容（必需） |
| `description` | `Option<Element>` | `None` | 可选的详细描述 |
| `show_icon` | `bool` | `true` | 是否显示语义图标 |
| `closable` | `bool` | `false` | 是否可以关闭 |
| `on_close` | `Option<EventHandler<()>>` | `None` | 关闭按钮点击时的回调 |
| `icon` | `Option<Element>` | `None` | 可选的自定义图标元素 |
| `banner` | `bool` | `false` | 是否渲染为横幅（全宽、紧凑） |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |

### AlertType

- `Success` - 成功消息（绿色）
- `Info` - 信息消息（蓝色）
- `Warning` - 警告消息（橙色）
- `Error` - 错误消息（红色）

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Success,
        message: rsx!("操作成功完成！"),
    }
}
```

### 带描述

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Warning,
        message: rsx!("警告"),
        description: Some(rsx!("此操作无法撤销。")),
    }
}
```

### 可关闭的警告

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Info,
        message: rsx!("此警告可以关闭"),
        closable: true,
        on_close: Some(move |_| {
            println!("警告已关闭");
        }),
    }
}
```

### 横幅警告

```rust
use adui_dioxus::{Alert, AlertType};

rsx! {
    Alert {
        r#type: AlertType::Error,
        message: rsx!("系统错误"),
        banner: true,
    }
}
```

## 使用场景

- **表单验证**：显示验证错误或警告
- **系统通知**：显示系统范围的消息
- **成功反馈**：确认成功操作
- **错误消息**：显示错误信息
- **横幅警告**：页面顶部通知

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础警告类型
- ✅ 可关闭功能
- ✅ 横幅模式
- ✅ 自定义图标
- ⚠️ 操作按钮尚未实现
- ⚠️ 某些高级样式选项可能有所不同

