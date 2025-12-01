# Modal 对话框

## 概述

Modal 组件显示一个模态对话框用于用户交互。它支持标题、自定义页脚、加载状态和各种配置选项。

## API 参考

### ModalProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `open` | `bool` | - | 对话框是否可见（必需） |
| `title` | `Option<String>` | `None` | 头部显示的可选标题 |
| `footer` | `Option<Element>` | `None` | 自定义页脚内容（None 显示默认的确定/取消） |
| `footer_render` | `Option<Rc<dyn Fn(Element, FooterExtra) -> Element>>` | `None` | 自定义页脚渲染函数 |
| `show_footer` | `bool` | `true` | 是否显示页脚 |
| `on_ok` | `Option<EventHandler<()>>` | `None` | 用户确认时调用 |
| `on_cancel` | `Option<EventHandler<()>>` | `None` | 用户取消时调用 |
| `closable` | `bool` | `true` | 在右上角显示关闭按钮 |
| `closable_config` | `Option<ClosableConfig>` | `None` | 高级可关闭配置 |
| `mask_closable` | `bool` | `true` | 点击遮罩是否触发 on_cancel |
| `destroy_on_close` | `bool` | `false` | 关闭时从树中移除内容 |
| `destroy_on_hidden` | `bool` | `false` | 隐藏时移除内容 |
| `force_render` | `bool` | `false` | 即使不可见也强制渲染 |
| `width` | `Option<f32>` | `None` | 固定宽度（像素） |
| `width_responsive` | `Option<HashMap<String, f32>>` | `None` | 响应式宽度配置 |
| `centered` | `bool` | `false` | 是否垂直居中对话框 |
| `confirm_loading` | `bool` | `false` | 确定按钮是否处于加载状态 |
| `ok_text` | `Option<String>` | `None` | 确定按钮文本 |
| `cancel_text` | `Option<String>` | `None` | 取消按钮文本 |
| `ok_type` | `Option<ButtonType>` | `None` | 确定按钮类型 |
| `keyboard` | `bool` | `true` | 启用键盘（Esc 关闭） |
| `close_icon` | `Option<Element>` | `None` | 自定义关闭图标 |
| `after_close` | `Option<EventHandler<()>>` | `None` | 对话框关闭后的回调 |
| `after_open_change` | `Option<EventHandler<bool>>` | `None` | 打开状态改变时的回调 |
| `class` | `Option<String>` | `None` | 根元素的额外 CSS 类 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |
| `class_names` | `Option<ModalClassNames>` | `None` | 语义类名 |
| `styles` | `Option<ModalStyles>` | `None` | 语义样式 |
| `get_container` | `Option<String>` | `None` | 自定义容器选择器 |
| `z_index` | `Option<i32>` | `None` | 自定义 z-index |
| `mask` | `Option<MaskConfig>` | `None` | 遮罩配置 |
| `loading` | `bool` | `false` | 整个对话框的加载状态 |
| `children` | `Element` | - | 对话框内容（必需） |

### ModalType

- `Info` - 信息对话框
- `Success` - 成功对话框
- `Error` - 错误对话框
- `Warning` - 警告对话框
- `Confirm` - 确认对话框

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Modal, Button, ButtonType};
use dioxus::prelude::*;

let is_open = use_signal(|| false);

rsx! {
    div {
        Button {
            onclick: move |_| {
                is_open.set(true);
            },
            "打开对话框"
        }
        Modal {
            open: *is_open.read(),
            title: Some("对话框标题".to_string()),
            on_cancel: Some(move |_| {
                is_open.set(false);
            }),
            on_ok: Some(move |_| {
                println!("确定被点击");
                is_open.set(false);
            }),
            "对话框内容在这里"
        }
    }
}
```

### 自定义页脚

```rust
use adui_dioxus::{Modal, Button, ButtonType};

rsx! {
    Modal {
        open: true,
        title: Some("自定义页脚".to_string()),
        footer: Some(rsx! {
            Button { "自定义按钮" }
        }),
        "内容"
    }
}
```

### 居中对话框

```rust
use adui_dioxus::Modal;

rsx! {
    Modal {
        open: true,
        title: Some("居中对话框".to_string()),
        centered: true,
        "居中内容"
    }
}
```

### 加载状态

```rust
use adui_dioxus::Modal;

rsx! {
    Modal {
        open: true,
        title: Some("加载中的对话框".to_string()),
        confirm_loading: true,
        on_ok: Some(move |_| {
            // 异步操作
        }),
        "内容"
    }
}
```

## 使用场景

- **确认**：确认用户操作
- **表单**：在对话框中显示表单
- **详情**：显示详细信息
- **提醒**：显示重要提醒

## 与 Ant Design 6.0.0 的差异

- ✅ 基础对话框功能
- ✅ 自定义页脚
- ✅ 加载状态
- ✅ 居中定位
- ✅ 遮罩配置
- ⚠️ 某些高级功能可能有所不同
- ⚠️ 静态方法变体（Modal.info 等）通过 use_modal 钩子访问

