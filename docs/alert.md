# Alert：页面级提示（MVP）

> 实现位置：`src/components/alert.rs`
>
> 示例：`examples/alert_demo.rs`

## 1. 设计目标

Alert 用于在页面中展示重要提示或警告信息，适合承载表单顶部告警、权限提示、全局配置信息等。

本版 Alert 提供一个对齐 Ant Design 语义的 **MVP 子集**：

- 支持四种语义类型：`success` / `info` / `warning` / `error`；
- 支持可选图标与可关闭行为；
- 支持简单 banner 模式（整行横幅样式）。

暂不实现复杂的描述布局、动画配置和所有可定制位点，重点满足中后台常见用法。

---

## 2. AlertProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    pub r#type: AlertType,
    pub message: Element,
    pub description: Option<Element>,
    pub show_icon: bool,
    pub closable: bool,
    pub on_close: Option<EventHandler<()>>,
    pub icon: Option<Element>,
    pub banner: bool,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

- `r#type: AlertType`
  - 语义类型，决定颜色与默认图标：
    - `AlertType::Success`
    - `AlertType::Info`
    - `AlertType::Warning`
    - `AlertType::Error`
  - 对应样式类：`.adui-alert-success` / `.adui-alert-info` / `.adui-alert-warning` / `.adui-alert-error`。
- `message: Element`
  - 主提示内容（标题行），通常是一段简短文字。
- `description: Option<Element>`
  - 可选描述文本，用于承载更详细的说明。
- `show_icon: bool`
  - 是否展示语义图标；
  - 默认为 `true`。
- `closable: bool`
  - 是否展示右侧关闭按钮；
  - 为 `true` 时，会渲染一个 `X` 按钮并允许用户关闭 Alert。
- `on_close: Option<EventHandler<()>>`
  - 用户点击关闭按钮时触发；
  - 同时，组件会在内部通过 Signal 将自身隐藏（非受控关闭）——外层如果使用受控模式，可以在回调中移除对应 Alert。
- `icon: Option<Element>`
  - 自定义图标节点；
  - 若提供，则覆盖默认的语义图标。
- `banner: bool`
  - 为 `true` 时使用横幅样式：去除左右圆角，适合放在页面顶部。
- `class` / `style`
  - 附加类名与内联样式，作用于根 `div`。

---

## 3. DOM 结构与样式类

典型结构：

```html
<div class="adui-alert adui-alert-success">
  <div class="adui-alert-icon">...</div>        <!-- 可选 -->
  <div class="adui-alert-content">
    <div class="adui-alert-message">操作成功</div>
    <div class="adui-alert-description">详细说明...</div>  <!-- 可选 -->
  </div>
  <button class="adui-alert-close-icon">×</button> <!-- 可选 -->
</div>
```

主要类名（定义于 `src/theme.rs` 的 `adui_alert_style!` 中）：

- `.adui-alert`：根容器，水平布局；
- `.adui-alert-icon`：图标区域；
- `.adui-alert-content`：文本区域；
- `.adui-alert-message`：主提示文本（标题行）；
- `.adui-alert-description`：描述文本；
- `.adui-alert-close-icon`：关闭按钮；
- `.adui-alert-success` / `-info` / `-warning` / `-error`：不同语义配色；
- `.adui-alert-banner`：横幅模式。

---

## 4. 示例：基础用法与可关闭提示

摘自 `examples/alert_demo.rs`：

```rust
#[component]
fn AlertDemoShell() -> Element {
    let mut show_success = use_signal(|| true);

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Alert demo" }
            p { "基础四种类型和可关闭行为示例。" }

            div { style: "margin-bottom: 16px; display: flex; gap: 8px;",
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| show_success.set(true),
                    "重置成功提示"
                }
            }

            if *show_success.read() {
                Alert {
                    r#type: AlertType::Success,
                    show_icon: true,
                    closable: true,
                    on_close: move |_| show_success.set(false),
                    message: rsx!("操作成功"),
                    description: Some(rsx!("用于展示表单提交或任务完成后的成功提示。")),
                }
            }

            Alert {
                r#type: AlertType::Info,
                show_icon: true,
                message: rsx!("信息提示"),
                description: Some(rsx!("普通信息提示，可用于说明当前页面状态。")),
            }

            Alert {
                r#type: AlertType::Warning,
                show_icon: true,
                message: rsx!("警告"),
                description: Some(rsx!("用于提醒潜在风险或需要用户留意的设置。")),
            }

            Alert {
                r#type: AlertType::Error,
                show_icon: true,
                message: rsx!("错误"),
                description: Some(rsx!("用于展示严重错误或阻断性问题。")),
                banner: true,
            }
        }
    }
}
```

---

## 5. 与 Message / Notification 的使用边界

- `Alert` 更适合：
  - 作为页面内容的一部分，长时间停留在页面上；
  - 提示当前页面状态、权限问题、配置变更等；
  - 放在 Form 顶部、Card 内、Tabs 的面板内等位置。
- `Message` / `Notification` 更适合：
  - 短暂的全局反馈（如表单提交成功、操作失败提示）；
  - 需要覆盖当前页面的即时反馈，不依赖特定容器位置；
  - 带入口的全局通知（Notification）。

推荐模式：

- 表单提交失败：在 Form 顶部使用 Alert 展示校验/业务错误详情，同时配合 Message/Notification 给出简短提示；
- 多步流程：在 Steps 对应的面板中使用 Alert 提醒当前步骤的风险或注意事项；
- 全局配置：在布局顶部使用 banner 模式的 Alert 提示当前环境、试用状态等。