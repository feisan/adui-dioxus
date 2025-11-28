# Progress：进度条（MVP）

> 实现位置：`src/components/progress.rs`
>
> 示例：`examples/progress_demo.rs`

## 1. 设计目标

Progress 用于展示任务或流程的整体完成度，常见于：

- 上传/下载任务进度；
- 批量处理任务的完成比例；
- 表单/操作向导中，每步完成度的可视化展示。

本版 Progress 提供一个对齐 Ant Design 语义的 **MVP 子集**：

- 支持线形进度条（`line`）和简化版圆形进度（`circle`）；
- 支持 `percent` 与基础 `status`（normal/success/exception/active）；
- 支持是否显示进度文本 `show_info`。

暂不实现仪表盘（dashboard）、渐变色、steps 模式等高级能力，优先覆盖常见用法。

---

## 2. ProgressStatus / ProgressType / ProgressProps

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressStatus {
    Normal,
    Success,
    Exception,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressType {
    Line,
    Circle,
}

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    pub percent: f32,
    pub status: Option<ProgressStatus>,
    pub show_info: bool,
    pub r#type: ProgressType,
    pub stroke_width: Option<f32>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

字段说明：

- `percent: f32`
  - 进度百分比，组件内部会将其 clamp 到 `[0.0, 100.0]` 范围；
  - `NaN` 会按 `0.0` 处理。
- `status: Option<ProgressStatus>`
  - 进度状态：
    - `Normal`：默认状态；
    - `Success`：成功状态；
    - `Exception`：异常状态（错误/失败）；
    - `Active`：进行中状态（样式上同 normal，但保留扩展钩子）；
  - 若未指定且 `percent >= 100`，组件会自动使用 `Success`；
  - 仅影响样式（颜色等），不会自动改变 percent。
- `show_info: bool`
  - 是否显示文字形式的百分比（如 `50%`）；
  - 默认 `true`。
- `r#type: ProgressType`
  - 进度类型：
    - `Line`：线形进度条（默认）；
    - `Circle`：简化圆形进度展示（基于 conic-gradient 渲染）；
- `stroke_width: Option<f32>`
  - 控制线形进度条的高度或圆形边框宽度；
  - 未指定时使用默认值（线形约 6px，圆形约 6px）。
- `class` / `style`
  - 附加类名与内联样式，作用于根元素。

---

## 3. DOM 结构与样式类

### 3.1 线形进度条（Line）

结构：

```html
<div class="adui-progress adui-progress-line adui-progress-status-normal">
  <div class="adui-progress-outer">
    <div class="adui-progress-inner">
      <div class="adui-progress-bg" style="width: 50%; height: 6px;"></div>
    </div>
  </div>
  <span class="adui-progress-text">50%</span>
</div>
```

主要类名：

- `.adui-progress`：根容器，行内布局；
- `.adui-progress-line`：线形进度标记；
- `.adui-progress-status-normal/success/exception/active`：状态类；
- `.adui-progress-outer` / `.adui-progress-inner`：外层/内层轨道；
- `.adui-progress-bg`：实际填充的进度条；
- `.adui-progress-text`：右侧百分比文本。

### 3.2 圆形进度条（Circle）

结构（简化版本）：

```html
<div class="adui-progress adui-progress-circle adui-progress-status-normal">
  <div class="adui-progress-circle-inner" style="width:80px; height:80px; border-width:6px; background: conic-gradient(currentColor 75%, rgba(0,0,0,0.06) 0);"></div>
  <div class="adui-progress-text">75%</div>
</div>
```

说明：

- 使用 `conic-gradient` 绘制圆形进度，依赖现代浏览器；
- 外层 `.adui-progress-circle-inner` 通过 inline style 设置尺寸、边框宽度与渐变；
- 文本与线形形式一致，显示百分比。

样式定义位于 `src/theme.rs` 的 `adui_progress_style!` 宏中，核心类包括：

- `.adui-progress` / `.adui-progress-outer` / `.adui-progress-inner` / `.adui-progress-bg` / `.adui-progress-text`；
- `.adui-progress-status-success/exception/active`：状态色控制；
- `.adui-progress-circle` / `.adui-progress-circle-inner`：圆形布局与绘制。

---

## 4. 示例：基础用法

摘自 `examples/progress_demo.rs`：

```rust
#[component]
fn ProgressDemoShell() -> Element {
    let mut percent = use_signal(|| 30.0f32);

    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Progress demo" }
            p { "展示线形和圆形进度条，以及简单的状态切换。" }

            div { style: "margin-bottom: 16px; display: flex; gap: 8px;",
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| {
                        let mut value = percent;
                        let next = (*value.read() + 10.0).min(100.0);
                        value.set(next);
                    },
                    "增加 10%"
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: move |_| {
                        let mut value = percent;
                        value.set(0.0);
                    },
                    "重置"
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 12px; max-width: 360px;",
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Line,
                }
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Line,
                    status: Some(ProgressStatus::Exception),
                }
                div { style: "margin-top: 16px;", "圆形进度：" }
                Progress {
                    percent: *percent.read(),
                    show_info: true,
                    r#type: ProgressType::Circle,
                }
            }
        }
    }
}
```

---

## 5. 与 Steps / Result 的组合建议

- 长任务进度：
  - 使用线形 Progress 展示整体进度，配合 Steps 显示当前步骤；
  - 任务成功后，将 `percent` 设为 100，并切换到 `ResultStatus::Success` 结果页；
  - 任务失败时，将 Progress 状态设为 `Exception`，并用 Result/Alert 解释失败原因。
- 多步表单：
  - 在每个步骤面板顶部使用 Progress 表示整体完成度（例如当前步骤/总步骤）；
  - 与 Steps 组合，形成“步骤条 + 进度条 + 表单内容”的布局。

与 Ant Design 的差异：

- 当前版本不支持：
  - `dashboard` 类型、环形半仪表盘；
  - 渐变色 `strokeColor`/`trailColor` 的完全对齐；
  - `steps` 模式（将进度拆分为多个块状步进）；
- 目前 `circle` 使用 CSS `conic-gradient` 简化实现，不适合所有旧浏览器；
- 若需要更细致的视觉对齐，可以在后续计划中扩展 SVG 圆环绘制与更多样式钩子。