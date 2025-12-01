# Notification 通知提醒框

## 概述

Notification 组件在页面角落显示通知消息。它与 Message 类似，但支持标题、描述和不同的位置。

## API 参考

### NotificationApi

Notification API 通过 `use_notification` 钩子访问：

```rust
let notification = use_notification();
```

#### 方法

- `open(config: NotificationConfig) -> OverlayKey` - 使用完整配置打开通知
- `info(title: impl Into<String>, description: Option<String>) -> OverlayKey` - 显示信息通知
- `success(title: impl Into<String>, description: Option<String>) -> OverlayKey` - 显示成功通知
- `error(title: impl Into<String>, description: Option<String>) -> OverlayKey` - 显示错误通知
- `warning(title: impl Into<String>, description: Option<String>) -> OverlayKey` - 显示警告通知

### NotificationConfig

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `String` | - | 通知标题（必需） |
| `description` | `Option<String>` | `None` | 可选的描述文本 |
| `type` | `NotificationType` | `NotificationType::Info` | 通知类型 |
| `placement` | `NotificationPlacement` | `NotificationPlacement::TopRight` | 屏幕上的位置 |
| `duration` | `f32` | `4.5` | 自动关闭延迟（秒，0 表示不自动关闭） |
| `icon` | `Option<Element>` | `None` | 自定义图标元素 |
| `class` | `Option<String>` | `None` | 额外的 CSS 类 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_click` | `Option<EventHandler<()>>` | `None` | 通知被点击时的回调 |
| `key` | `Option<String>` | `None` | 用于程序化更新的唯一键 |

### NotificationType

- `Info` - 信息通知
- `Success` - 成功通知
- `Warning` - 警告通知
- `Error` - 错误通知

### NotificationPlacement

- `TopRight` - 右上角（默认）
- `TopLeft` - 左上角
- `BottomRight` - 右下角
- `BottomLeft` - 左下角

## 使用示例

### 基础用法

```rust
use adui_dioxus::use_notification;

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.info(
                "通知标题".to_string(),
                Some("这是描述".to_string())
            );
        },
        "显示通知"
    }
}
```

### 成功通知

```rust
use adui_dioxus::use_notification;

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.success(
                "成功".to_string(),
                Some("操作成功完成".to_string())
            );
        },
        "成功"
    }
}
```

### 自定义位置

```rust
use adui_dioxus::{use_notification, NotificationConfig, NotificationType, NotificationPlacement};

let notification = use_notification();

rsx! {
    Button {
        onclick: move |_| {
            notification.open(NotificationConfig {
                title: "自定义位置".to_string(),
                description: Some("左下角".to_string()),
                r#type: NotificationType::Info,
                placement: NotificationPlacement::BottomLeft,
                ..Default::default()
            });
        },
        "自定义位置"
    }
}
```

## 使用场景

- **系统通知**：显示系统范围的通知
- **用户反馈**：为用户操作提供反馈
- **提醒**：显示重要提醒
- **更新**：通知用户更新或更改

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有通知类型
- ✅ 标题和描述
- ✅ 自定义位置
- ✅ 自定义持续时间
- ⚠️ 关闭按钮尚未实现
- ⚠️ 某些高级样式选项可能有所不同

