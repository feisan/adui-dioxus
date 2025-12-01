# App 应用

## 概述

App 组件是一个顶级容器，它连接 OverlayManager 并通过上下文公开全局反馈 API（Message、Notification、Modal）。它应该包装您的应用程序根。

## API 参考

### AppProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `class` | `Option<String>` | `None` | 额外的 CSS 类 |
| `style` | `Option<String>` | `None` | 额外的内联样式 |
| `children` | `Element` | - | 应用程序内容（必需） |

## 使用示例

### 基础 App

```rust
use adui_dioxus::App;

rsx! {
    App {
        div {
            "您的应用程序内容"
        }
    }
}
```

### 使用 Message API

```rust
use adui_dioxus::{App, use_message};

fn MyComponent() -> Element {
    let message = use_message();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(msg) = message {
                    msg.success("操作成功");
                }
            },
            "显示消息"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

### 使用 Notification API

```rust
use adui_dioxus::{App, use_notification};

fn MyComponent() -> Element {
    let notification = use_notification();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(notif) = notification {
                    notif.info("通知", "这是一条通知");
                }
            },
            "显示通知"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

### 使用 Modal API

```rust
use adui_dioxus::{App, use_modal};

fn MyComponent() -> Element {
    let modal = use_modal();
    
    rsx! {
        Button {
            onclick: move |_| {
                if let Some(m) = modal {
                    m.open();
                }
            },
            "打开模态框"
        }
    }
}

rsx! {
    App {
        MyComponent {}
    }
}
```

## Hooks

### use_app()

返回完整的 App 上下文值。在 App 子树外使用时返回默认值。

### use_message()

访问 Message API 的便捷钩子。在 App 外使用时返回 `None`。

### use_notification()

访问 Notification API 的便捷钩子。在 App 外使用时返回 `None`。

### use_modal()

访问 Modal API 的便捷钩子。在 App 外使用时返回 `None`。

## 使用场景

- **应用程序根**：包装您的应用程序根
- **全局反馈**：启用 Message、Notification 和 Modal API
- **覆盖层管理**：全局管理覆盖层
- **上下文提供**：为全局 API 提供上下文

## 与 Ant Design 6.0.0 的差异

- ✅ Message API 支持
- ✅ Notification API 支持
- ✅ Modal API 支持
- ✅ 覆盖层管理
- ⚠️ 相比 Ant Design 的完整 App 组件已简化
- ⚠️ 某些高级功能可能有所不同

