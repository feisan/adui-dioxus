# Badge：角标与小红点（MVP）

> 实现位置：`src/components/badge.rs`
>
> 示例：`examples/badge_demo.rs`

## 1. 设计目标

Badge 用于在元素角落展示数量信息或状态标记，常见于按钮、图标、头像、浮动按钮等上方的小红点或数字角标。

当前实现为基础版：

- 支持 `count` 数字角标与 `overflow_count` 上限；
- 支持 `dot` 小红点；
- 支持简单 `status` 状态色（default/success/warning/error）；
- 未实现 Ribbon、复杂文本位置调整等高级特性。

---

## 2. BadgeStatus 与 BadgeProps

### 2.1 BadgeStatus

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BadgeStatus {
    Default,
    Success,
    Warning,
    Error,
}
```

- 对应基础状态色：
  - `Default`：默认样式；
  - `Success`：成功状态；
  - `Warning`：警告状态；
  - `Error`：错误状态；
- 在当前实现中，仅用于修改角标/小红点的背景色。

### 2.2 BadgeProps

```rust
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub count: Option<u32>,
    pub overflow_count: u32,
    pub dot: bool,
    pub show_zero: bool,
    pub status: Option<BadgeStatus>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
}
```

字段说明：

- `count`：
  - 需要展示的数字；
  - 若 `None` 且 `dot=false`，则不展示角标；
- `overflow_count`：
  - 数值上限，默认 `99`；
  - 当 `count > overflow_count` 时展示 `"overflow+"`（如 `99+`）；
- `dot`：
  - 是否展示小红点，而非数字；
  - `dot=true` 时忽略 `count` 文本，仅展示圆点；
- `show_zero`：
  - 当 `count = Some(0)` 时是否展示角标；
  - 默认 `false`，即 0 不展示；
- `status`：
  - 可选状态色，影响角标背景：`Success`/`Warning`/`Error`；
- `class` / `style`：
  - 作用于根 `.adui-badge` 元素的附加类名与内联样式；
- `children`：
  - 被包裹的目标元素，如按钮、图标、浮动按钮等；
  - 为空时 Badge 仍会渲染角标，但通常建议提供子元素以保证语义清晰。

---

## 3. 渲染结构与样式类

UI 结构（简化）：

```html
<span class="adui-badge adui-badge-status-success?">
  <button>消息</button>
  <span class="adui-badge-count">5</span>
</span>
```

主要类名（`src/theme.rs` 的 `adui_badge_style!`）：

- `.adui-badge`：根容器，`position: relative`，用于定位角标；
- `.adui-badge-count`：数字角标圆角矩形，右上角定位；
- `.adui-badge-dot`：小红点圆形标记；
- `.adui-badge-status-success` / `.adui-badge-status-warning` / `.adui-badge-status-error`：修改 count/dot 的背景色，使其匹配成功/警告/错误状态颜色。

---

## 4. 示例：按钮角标、小红点与状态标记

摘自 `examples/badge_demo.rs`：

```rust
#[component]
fn BadgeDemoShell() -> Element {
    rsx! {
        // 挂在按钮上的数字角标
        Badge {
            count: Some(5),
            children: Some(rsx!(
                Button { r#type: ButtonType::Default, "消息" }
            )),
        }
        Badge {
            count: Some(120),
            overflow_count: 99,
            children: Some(rsx!(
                Button { r#type: ButtonType::Default, "通知" }
            )),
        }

        // 小红点
        Badge {
            dot: true,
            children: Some(rsx!(
                Button { r#type: ButtonType::Default, "待处理" }
            )),
        }

        // 状态色示例
        Badge {
            count: Some(1),
            status: Some(BadgeStatus::Success),
            children: Some(rsx!("成功")),
        }
    }
}
```

---

## 5. 与其他组件的协同

- 与 Button/Icon：
  - 将 `Button` 或 `Icon` 作为 `children` 包裹，Badge 负责在右上角展示数字或小红点；
  - 常用于通知、消息、待办按钮；
- 与 FloatButton：
  - 可将 `FloatButton` 作为 `children`，在右下角悬浮按钮上展示待处理数量；
- 与 Avatar/Menu：
  - 后续可在 Avatar 或 MenuItem 上挂 Badge，表达未读消息或新功能提示；

---

## 6. 与 Ant Design 的差异与后续规划

与 Ant Design 6.x 的 Badge 相比，当前实现为裁剪版：

- 暂未支持：
  - `Ribbon` 装饰条；
  - 自定义 `offset` 精细位置调整；
  - 任意自定义颜色字符串（当前仅通过 status 控制）；
- 行为差异：
  - 未实现所有 size 变体，仅提供默认样式；

后续扩展方向：

- 根据需求增加 `offset` 支持，允许精细调整角标位置；
- 提供更丰富的状态和自定义颜色接口；
- 引入 Ribbon 能力，用于 Card/Panel 边角装饰。
