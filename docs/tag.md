# Tag：状态标签（MVP）

> 实现位置：`src/components/tag.rs`
>
> 示例：`examples/tag_demo.rs`

## 1. 设计目标

Tag 用于表达状态/类别/筛选条件，是 Table/List 中常见的装饰组件，也可以单独用于标签云、搜索条件展示等场景。

当前实现为基础版：

- 支持少量预设颜色（default/primary/success/warning/error）；
- 支持 `closable` 可关闭标签；
- 支持简单 `checkable` 标签（可选中，受控/非受控模式）；
- 不支持 AntD 中全部 variant/status/icon/href 能力，仅覆盖常用子集。

---

## 2. TagColor 与 TagProps

### 2.1 TagColor

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagColor {
    Default,
    Primary,
    Success,
    Warning,
    Error,
}
```

- 对应 AntD 的部分预设状态色：
  - `Default`：默认边框 + 次要文本色；
  - `Primary`：主色调标签；
  - `Success` / `Warning` / `Error`：成功/警告/错误状态标签。

### 2.2 TagProps

```rust
#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    pub color: Option<TagColor>,
    pub closable: bool,
    pub on_close: Option<EventHandler<()>>,
    pub checkable: bool,
    pub checked: Option<bool>,
    pub default_checked: Option<bool>,
    pub on_change: Option<EventHandler<bool>>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Element,
}
```

字段说明：

- `color`：
  - 预设颜色枚举，对应不同的边框/文字/背景；
  - 未指定时使用默认样式；
- `closable`：
  - 是否渲染右侧关闭按钮；
  - 点击关闭按钮后会触发 `on_close` 回调；
- `on_close()`：
  - 关闭按钮点击回调，事件会 `stop_propagation()`，不会触发 checkable 点击；
- `checkable`：
  - 是否为可选中标签，点击时切换选中状态；
- `checked` / `default_checked`：
  - `checked` 存在时，Tag 处于 **受控模式**，内部不管理选中状态；
  - 否则使用 `default_checked` 作为非受控初始值，内部通过 Signal 管理；
- `on_change(checked)`：
  - 当选中状态变化时触发，无论受控/非受控都会回调；
- `class` / `style`：
  - 附加类名与内联样式；
- `children`：
  - 标签文本/内容。

---

## 3. 渲染结构与样式类

UI 结构（简化）：

```html
<span class="adui-tag adui-tag-primary adui-tag-checkable adui-tag-checkable-checked">
  标签文本
  <button class="adui-tag-close">×</button>
</span>
```

主要类名（`src/theme.rs` 的 `adui_tag_style!`）：

- `.adui-tag`：标签根样式，inline-flex，对齐、内边距、圆角与默认背景；
- `.adui-tag-default`：默认边框 + 次要文字颜色；
- `.adui-tag-primary` / `.adui-tag-success` / `.adui-tag-warning` / `.adui-tag-error`：不同状态色对应的边框/文字/背景；
- `.adui-tag-checkable`：标记为可选标签，cursor: pointer；
- `.adui-tag-checkable-checked`：选中状态，主色背景 + 白色文字；
- `.adui-tag-close`：关闭按钮样式，使用内置 `IconKind::Close` 渲染小型关闭图标。

---

## 4. 示例：预设颜色、可关闭与可选标签

摘自 `examples/tag_demo.rs`：

```rust
#[component]
fn TagDemoShell() -> Element {
    let mut checked = use_signal(|| true);

    rsx! {
        // 预设颜色标签
        Tag { children: rsx!("Default") }
        Tag { color: Some(TagColor::Primary), children: rsx!("Primary") }
        Tag { color: Some(TagColor::Success), children: rsx!("Success") }
        Tag { color: Some(TagColor::Warning), children: rsx!("Warning") }
        Tag { color: Some(TagColor::Error), children: rsx!("Error") }

        // 可关闭标签
        Tag { closable: true, children: rsx!("Closable") }
        Tag { color: Some(TagColor::Primary), closable: true, children: rsx!("Primary closable") }

        // 受控可选中标签
        Tag {
            checkable: true,
            checked: Some(*checked.read()),
            on_change: move |next| checked.set(next),
            children: rsx!("Checkable")
        }

        // 非受控可选中标签
        Tag {
            checkable: true,
            default_checked: Some(true),
            children: rsx!("Uncontrolled checkable")
        }
    }
}
```

---

## 5. 与 Table/List 的协同

- Table 列：
  - 建议在 Table 行中使用 Tag 表达状态，例如订单状态（已完成/进行中/已关闭）；
  - 可通过 `TagColor::Success/Warning/Error` 对应成功/警告/错误；
- List 项：
  - 在 List 的 item 内容中组合多个 Tag 表示标签/分类；
- 筛选条件：
  - 使用 `checkable` Tag 表示当前选中的筛选条件，`checked` 由 Form 或外部状态管理；
- 与 Card：
  - 在 Card 内放置 Tag，标记卡片的类型或状态（例如「进行中项目」「归档项目」）。

---

## 6. 与 Ant Design 的差异与后续规划

与 Ant Design 6.x 的 Tag 相比，当前实现为裁剪版：

- 暂未支持：
  - `variant`（filled/outlined/solid）完整集合；
  - 自定义 `icon`、`href`、`target` 等；
  - `CheckableTag`/`CheckableTagGroup` 独立导出 API（当前直接通过 `checkable` + `on_change` 覆盖简单场景）。
- 颜色：
  - 当前仅支持少量预设色，未开放任意字符串颜色；

后续扩展方向：

- 根据实际需求扩展 `TagColor` 范围，并允许直接传字符串颜色；
- 引入更细粒度的 variant 支持（filled/outlined 等），对齐 AntD 视觉；
- 提供独立的 `CheckableTag`/`CheckableTagGroup`，简化多选标签组用法。
