# Steps：流程步骤条（MVP）

> 实现位置：`src/components/steps.rs`
>
> 示例：`examples/steps_demo.rs`

## 1. 设计目标

Steps 用于展示多步流程的当前进度和整体结构，常见于：

- 多步表单（注册、开通向导等）；
- 审批/配置流程；
- 由多个阶段组成的操作任务。

本版 Steps 提供一个对齐 Ant Design 语义的 **MVP 子集**：

- 支持水平/垂直两种方向（默认水平）；
- 支持 `current` 与 `default_current`，并通过 `on_change` 通知外层；
- 支持简单状态：等待/进行中/已完成/错误；
- 不实现 progress-dot、导航型 steps 等高级变体。

---

## 2. StepStatus / StepsDirection / StepItem / StepsProps

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepsDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, PartialEq)]
pub struct StepItem {
    pub key: String,
    pub title: Element,
    pub description: Option<Element>,
    pub status: Option<StepStatus>,
    pub disabled: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    pub items: Vec<StepItem>,
    pub current: Option<usize>,
    pub default_current: Option<usize>,
    pub on_change: Option<EventHandler<usize>>,
    pub direction: Option<StepsDirection>,
    pub size: Option<ComponentSize>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

说明：

- `StepStatus`：单个步骤的状态：
  - `Wait`：尚未到达；
  - `Process`：当前步骤；
  - `Finish`：已完成；
  - `Error`：错误状态。
- `StepsDirection`：步骤条方向：
  - `Horizontal`：水平步骤条（默认）；
  - `Vertical`：垂直步骤条。
- `StepItem`：步骤定义：
  - `key`：用于标识步骤（当前实现主要用于测试和扩展，DOM 上使用 `idx` 作为 key）；
  - `title`：步骤标题；
  - `description`：可选描述内容；
  - `status`：可选显示状态，若未提供则由组件根据 `current` 推导；
  - `disabled`：是否禁用（禁用时点击不触发切换）。
- `StepsProps`：
  - `items`：步骤列表；
  - `current`：受控当前步骤索引（从 0 开始）；
  - `default_current`：非受控初始步骤索引；
  - `on_change(next)`：当点击步骤项时触发，参数为目标步骤索引；
  - `direction`：方向（默认 `Horizontal`）；
  - `size: Option<ComponentSize>`：控制标题字号（Small/Middle/Large）；
  - `class` / `style`：作用于根 `ol` 的附加类名与内联样式。

状态推导逻辑：

```rust
fn effective_status(index: usize, current: usize, explicit: Option<StepStatus>) -> StepStatus {
    if let Some(st) = explicit {
        return st;
    }
    if index < current {
        StepStatus::Finish
    } else if index == current {
        StepStatus::Process
    } else {
        StepStatus::Wait
    }
}
```

---

## 3. DOM 结构与样式类

结构示例（水平步骤条）：

```html
<ol class="adui-steps adui-steps-horizontal">
  <li class="adui-steps-item adui-steps-status-finish">
    <div class="adui-steps-item-icon"><span class="adui-steps-item-index">1</span></div>
    <div class="adui-steps-item-content">
      <div class="adui-steps-item-title">步骤一</div>
      <div class="adui-steps-item-description">填写信息</div>
    </div>
  </li>
  <li class="adui-steps-item adui-steps-status-process adui-steps-item-current">...</li>
  <li class="adui-steps-item adui-steps-status-wait">...</li>
</ol>
```

主要类名（定义于 `src/theme.rs` 的 `adui_steps_style!`）：

- `.adui-steps`：根容器，`ol` 元素；
- `.adui-steps-horizontal` / `.adui-steps-vertical`：方向类；
- `.adui-steps-item`：单个步骤项；
- `.adui-steps-item-icon` / `.adui-steps-item-index`：圆形步骤序号；
- `.adui-steps-item-content`：内容容器；
- `.adui-steps-item-title`：标题；
- `.adui-steps-item-description`：描述；
- `.adui-steps-status-wait` / `-process` / `-finish` / `-error`：状态类，影响圆圈颜色；
- `.adui-steps-item-disabled`：禁用项；
- `.adui-steps-item-current`：当前步骤（目前主要作为样式钩子，可在后续扩展）。

---

## 4. 示例：多步流程

摘自 `examples/steps_demo.rs`：

```rust
#[component]
fn StepsDemoShell() -> Element {
    let mut current = use_signal(|| 0usize);

    let items = vec![
        StepItem::new("step1", rsx!("填写信息")),
        StepItem::new("step2", rsx!("确认")),
        StepItem::new("step3", rsx!("完成")),
    ];

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Steps demo" }
            p { "基础步骤条示例，演示受控 current 与简单结果视图。" }

            Steps {
                items: items.clone(),
                current: Some(*current.read()),
                on_change: move |next| current.set(next),
            }

            div { style: "margin-top: 24px; max-width: 480px;",
                match *current.read() {
                    0 => rsx! {
                        Card {
                            title: Some(rsx!("步骤一：填写信息")),
                            children: rsx!(
                                p { "在这一步填写基本信息（此处仅为示例文案）。" }
                                Button { r#type: ButtonType::Primary, onclick: move |_| current.set(1), "下一步" }
                            ),
                        }
                    },
                    1 => rsx! {
                        Card {
                            title: Some(rsx!("步骤二：确认")),
                            children: rsx!(
                                p { "请确认信息无误后继续。" }
                                Button { r#type: ButtonType::Default, onclick: move |_| current.set(0), "上一步" }
                                Button { r#type: ButtonType::Primary, onclick: move |_| current.set(2), "提交" }
                            ),
                        }
                    },
                    _ => rsx! {
                        Card {
                            title: Some(rsx!("步骤三：完成")),
                            children: rsx!(
                                Result {
                                    status: Some(ResultStatus::Success),
                                    title: Some(rsx!("提交成功")),
                                    sub_title: Some(rsx!("可以返回首页或继续操作。")),
                                    extra: Some(rsx!(
                                        Button { r#type: ButtonType::Primary, onclick: move |_| current.set(0), "重新开始" }
                                    )),
                                }
                            ),
                        }
                    },
                }
            }
        }
    }
}
```

该示例展示了：

- 使用 `Steps` 受控 `current`；
- 配合 `Card`/`Button` 组成多步表单的三个阶段；
- 最后一阶段使用 `Result` 展示成功结果。

---

## 5. 组合建议与与其它组件的关系

- 与 Form 组合：
  - 每个 Steps 步骤对应一个 Form 区域，`current` 用于切换显示的表单；
  - 通过 `on_change` 控制步骤切换，配合 Form 校验决定是否允许前进；
- 与 Progress/Result 组合：
  - 使用 Steps 展示离散步骤，Progress 展示整体完成度，Result 展示最终结果页；
- 与 Tabs/Card 组合：
  - 可将每个步骤内容放在不同 Tabs 面板或 Card 中，Steps 控制当前展示的面板。

与 Ant Design 的差异：

- 当前仅实现基础线型 Steps：
  - 不支持 progress-dot、navigation、panel 等高级 `type`；
  - 不支持内置图标 slot，需要时可以在 title/description 中自行组合 Icon；
- 状态推导逻辑简化为基于 `current` 的“前完成、中进行、后等待”模型，复杂场景可通过显式设置 `status` 来覆盖。