# Message 全局提示

## 概述

Message 组件在页面顶部显示全局反馈消息。通常通过 `use_message` 钩子使用，用于显示成功、错误、警告或信息消息。

## API 参考

### MessageApi

Message API 通过 `use_message` 钩子访问：

```rust
let message = use_message();
```

#### 方法

- `open(config: MessageConfig) -> OverlayKey` - 使用完整配置打开消息
- `info(content: impl Into<String>) -> OverlayKey` - 显示信息消息
- `success(content: impl Into<String>) -> OverlayKey` - 显示成功消息
- `error(content: impl Into<String>) -> OverlayKey` - 显示错误消息
- `warning(content: impl Into<String>) -> OverlayKey` - 显示警告消息
- `loading(content: impl Into<String>) -> OverlayKey` - 显示加载消息

### MessageConfig

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `content` | `String` | - | 消息内容（必需） |
| `type` | `MessageType` | `MessageType::Info` | 消息类型 |
| `duration` | `f32` | `3.0` | 自动关闭延迟（秒，0 表示不自动关闭） |
| `icon` | `Option<Element>` | `None` | 自定义图标元素 |
| `class` | `Option<String>` | `None` | 额外的 CSS 类 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `key` | `Option<String>` | `None` | 用于程序化更新的唯一键 |
| `on_click` | `Option<EventHandler<()>>` | `None` | 消息被点击时的回调 |

### MessageType

- `Info` - 信息消息
- `Success` - 成功消息
- `Warning` - 警告消息
- `Error` - 错误消息
- `Loading` - 加载消息

## 使用示例

### 基础用法

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.info("这是一条信息消息");
        },
        "显示信息"
    }
}
```

### 成功消息

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.success("操作成功完成！");
        },
        "成功"
    }
}
```

### 错误消息

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.error("发生错误");
        },
        "错误"
    }
}
```

### 自定义持续时间

```rust
use adui_dioxus::{use_message, MessageConfig, MessageType};

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.open(MessageConfig {
                content: "此消息停留 5 秒".to_string(),
                r#type: MessageType::Info,
                duration: 5.0,
                ..Default::default()
            });
        },
        "自定义持续时间"
    }
}
```

### 加载消息

```rust
use adui_dioxus::use_message;

let message = use_message();

rsx! {
    Button {
        onclick: move |_| {
            message.loading("处理中...");
        },
        "加载"
    }
}
```

## 使用场景

- **表单反馈**：显示验证错误或成功消息
- **操作反馈**：确认成功操作
- **错误通知**：显示错误消息
- **状态更新**：显示状态信息

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有消息类型
- ✅ 自定义持续时间
- ✅ 自定义图标
- ✅ 点击回调
- ⚠️ 消息堆叠/定位可能有所不同
- ⚠️ 某些高级样式选项可能有所不同

