# 浮层提示组件：Tooltip / Popover / Popconfirm / Dropdown（第一阶段：Tooltip）

本节记录「浮层提示家族」在 Dioxus 版本中的设计思路与当前实现进度。\
本轮（`plan/0008`）优先完成 `Tooltip` 与 `Popover` 的 MVP，实现基础浮层能力，并为后续 `Popconfirm` / `Dropdown` 预留扩展空间。

---

## Tooltip：轻量级气泡提示（MVP）

> 实现位置：`src/components/tooltip.rs`  
> 示例：`examples/tooltip_demo.rs`

### 设计目标

- 提供与 Ant Design 类似的轻量提示能力，覆盖最常见场景：
  - 鼠标 hover 时展示简单说明文案；
  - 点击触发的提示信息（如按钮上的解释说明）；
  - 支持基础的受控 `open` 模式。
- 复用现有 Overlay 体系：
  - 通过 `OverlayManager` 获取稳定的 `z-index`，与 Modal / Drawer / Select 下拉等浮层共享层级空间；
  - 统一触发/关闭行为（点击空白关闭、Esc 关闭）的大致策略。

### 核心类型与 Props

> 类型定义：`src/components/tooltip.rs`

主要类型：

- `TooltipPlacement`：浮层定位枚举（当前支持）：
  - `Top` / `Bottom` / `Left` / `Right`；
- `TooltipTrigger`：触发方式：
  - `Hover`（默认）；
  - `Click`。

`TooltipProps` 重要字段：

字段 | 类型 | 说明
--- | --- | ---
`title` | `Option<String>` | 简单文本内容，未提供 `content` 时生效。
`content` | `Option<Element>` | 自定义气泡内容节点，优先级高于 `title`。
`placement` | `Option<TooltipPlacement>` | 浮层相对触发元素的位置，默认 `Top`。
`trigger` | `TooltipTrigger` | 触发方式，默认 `Hover`。
`open` | `Option<bool>` | 受控打开状态；`Some` 时组件不再维护内部 state。
`default_open` | `Option<bool>` | 非受控初始打开状态，仅在 `open` 未设置时生效。
`on_open_change` | `Option<EventHandler<bool>>` | 打开状态变化回调（包含内部交互与关闭逻辑）。
`disabled` | `bool` | 是否禁用 Tooltip。
`class` | `Option<String>` | 触发区外层容器附加类名。
`overlay_class` | `Option<String>` | 气泡浮层容器附加类名。
`overlay_style` | `Option<String>` | 气泡浮层的内联样式（追加在默认定位样式之后）。
`children` | `Element` | 触发元素（例如 `Button` / `Icon` 等）。

### 触发与关闭行为

- **Hover 模式**（默认）：
  - 触发区 `onmouseenter` 时打开 Tooltip；
  - `onmouseleave` 时关闭 Tooltip；
  - 不依赖点击空白，行为与浏览器原生 `title` 类似。
- **Click 模式**：
  - 点击触发区时，切换 `open`（开关）；
  - 在非受控模式下，内部使用 `use_floating_close_handle`：
    - 点击空白区域会自动关闭 Tooltip；
    - 标记内部点击（触发区 / 浮层内部）不会触发关闭；
  - 按 `Esc` 键时会关闭 Tooltip，并触发一次 `on_open_change(false)`。

受控与非受控差异：

- 非受控模式（未传入 `open`）：
  - `Tooltip` 内部使用 `Signal<bool>` 维护可写的 `open_state`；
  - 所有交互（hover/click/Esc/点击空白）都会直接更新 `open_state`；
  - 如果提供了 `on_open_change`，会在内部状态更新后同步回调。
- 受控模式（传入 `open: Some(bool)`）：
  - `open` 作为单一数据源，组件不再修改内部 `open_state`；
  - 交互事件只通过 `on_open_change(next)` 通知外部，实际开关由父组件负责；
  - 当前版本中，**点击空白关闭只在非受控 + click 模式下生效**，受控模式下需要在回调里自行修改 `open`。

### 布局与样式

> 样式定义：`src/theme.rs` 中 `adui_tooltip_style!` 宏

基础类名：

- 触发区容器：`.adui-tooltip-root`；
- 浮层容器：`.adui-tooltip`；
- 内部内容容器：`.adui-tooltip-inner`。

MVP 样式特征：

- 背景为半透明黑色，白色文字，圆角小尺寸卡片；
- 默认字体为 12px，行高 1.4；
- 使用阴影 `box-shadow: 0 2px 8px rgba(0,0,0,0.15)` 以呼应其他浮层；
- 通过内联样式控制定位和 `z-index`：
  - `Top`：位于触发区上方，居中对齐；
  - `Bottom`：位于下方，居中对齐；
  - `Left` / `Right`：垂直居中，水平左右对齐；
  - 预留了 8px 间距（`margin-*: 8px`）。

### 示例：基础用法

摘自 `examples/tooltip_demo.rs`：

```rust
Tooltip {
    title: Some("默认 hover 提示".to_string()),
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Hover me"
        }
    },
}

Tooltip {
    title: Some("点击按钮切换 Tooltip".to_string()),
    trigger: TooltipTrigger::Click,
    placement: Some(TooltipPlacement::Bottom),
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Click me"
        }
    },
}
```

受控模式示例：

```rust
let mut controlled_open = use_signal(|| false);

Tooltip {
    title: Some("受控 Tooltip（由外部按钮控制 open）".to_string()),
    trigger: TooltipTrigger::Click,
    open: Some(*controlled_open.read()),
    on_open_change: move |next: bool| {
        controlled_open.set(next);
    },
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Controlled"
        }
    },
}

Button {
    r#type: ButtonType::Primary,
    onclick: move |_| controlled_open.set(false),
    "关闭受控 Tooltip"
}
```

### 与 Ant Design 的差异与后续扩展

与 Ant Design 6.x 相比，当前 Tooltip 版本是一个「裁剪后的 MVP」：

- 已实现：
  - `title` / `content` 插槽；
  - `placement` 基本方向（无 `*-start/end` 细分）；
  - `trigger` 的 `hover` / `click`；
  - 基础受控 `open` / `onOpenChange`；
  - 与 `OverlayManager` 的 z-index 统一管理。
- 暂未实现：
  - `mouseEnterDelay` / `mouseLeaveDelay` 等精细延迟控制；
  - 更复杂的对齐策略（如 `topLeft` / `bottomRight` 等）；
  - `getPopupContainer` 等容器自定义能力；
  - 全局配置项（例如通过 ConfigProvider 调整默认 placement）。

后续计划（与 `plan/0008` 对齐）：

- 在 Tooltip 基础上扩展 `Popover`（已完成 MVP，实现 `title` + `content` 结构化内容）；  
- 基于 `Popover` 封装 `Popconfirm`（内置确认/取消按钮与回调）；  
- 实现菜单型 `Dropdown`，复用相同的浮层触发与关闭机制。

---

## Popover：富内容气泡卡片（MVP）

> 实现位置：`src/components/popover.rs`  
> 示例：`examples/popover_demo.rs`

### 设计目标

- 在 Tooltip 的基础上，提供更丰富的内容容器：
  - 支持标题（`title`）与内容区（`content`）；
  - 内容区可以放任意 React 风格节点（按钮、链接、表单等）。
- 触发方式与 Tooltip 对齐，但默认使用 `click` 触发，更适合交互性强的内容。
- 复用相同的 Overlay 基础设施与关闭策略：
  - 使用 `OverlayKind::Popup` 和 `use_floating_layer` 管理 z-index；
  - 非受控 + click 模式下使用 `use_floating_close_handle` 支持点击空白关闭；
  - Esc 键关闭浮层。

### PopoverProps 主要字段

> 类型定义：`src/components/popover.rs`

字段 | 类型 | 说明
--- | --- | ---
`title` | `Option<Element>` | 标题区域节点（通常为一行文本或加粗标题）。
`content` | `Option<Element>` | 主内容区域（可以是任意布局）。
`placement` | `Option<TooltipPlacement>` | 浮层位置，语义与 Tooltip 一致，默认 `Top`。
`trigger` | `TooltipTrigger` | 触发方式，默认 `Click`，也支持 `Hover`。
`open` | `Option<bool>` | 受控打开状态。
`default_open` | `Option<bool>` | 非受控初始打开状态。
`on_open_change` | `Option<EventHandler<bool>>` | 打开状态变化回调。
`disabled` | `bool` | 是否禁用 Popover。
`class` | `Option<String>` | 触发区外层容器类名。
`overlay_class` | `Option<String>` | 浮层容器类名。
`overlay_style` | `Option<String>` | 浮层内联样式。
`children` | `Element` | 触发元素。

受控/非受控语义与 Tooltip 完全一致，这里不再赘述。

### 布局与样式

> 样式定义：`src/theme.rs` 中 `adui_tooltip_style!` 内的 `.adui-popover-*` 规则

基础类名：

- `.adui-popover-root`：触发区容器；
- `.adui-popover`：外层卡片容器；
- `.adui-popover-inner`：内层包裹；
- `.adui-popover-title`：标题区域；
- `.adui-popover-content`：内容区域。

默认样式：

- 使用卡片风格的背景与边框，继承 `--adui-color-bg-container` 与 `--adui-color-border`；
- 使用 `box-shadow: var(--adui-shadow-secondary)` 与 `var(--adui-radius-sm)` 做轻量卡片；
- 内部 padding 约为 `12px 16px`；
- 标题加粗并带有 8px 底部间距。

### 示例：基础用法

摘自 `examples/popover_demo.rs`：

```rust
// 默认点击触发
Popover {
    title: Some(rsx! { b { "标题" } }),
    content: Some(rsx! { p { "这是一个简单的 Popover 内容区域，可以放任意元素。" } }),
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Click Popover"
        }
    },
}

// Hover 触发
Popover {
    title: Some(rsx! { b { "Hover Popover" } }),
    content: Some(rsx! { p { "鼠标悬停时展示的 Popover。" } }),
    trigger: TooltipTrigger::Hover,
    placement: Some(TooltipPlacement::Right),
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Hover me"
        }
    },
}
```

受控模式示例：

```rust
let mut controlled_open = use_signal(|| false);

Popover {
    title: Some(rsx! { b { "受控 Popover" } }),
    content: Some(rsx! { p { "由外部按钮完全控制 open 状态。" } }),
    trigger: TooltipTrigger::Click,
    open: Some(*controlled_open.read()),
    on_open_change: move |next: bool| {
        controlled_open.set(next);
    },
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "Controlled"
        }
    },
}

Button {
    r#type: ButtonType::Primary,
    onclick: move |_| controlled_open.set(false),
    "关闭受控 Popover"
}
```

### 与 Tooltip 的关系

- Popover 与 Tooltip 共用：
  - `TooltipPlacement` / `TooltipTrigger` 两个枚举；
  - 相同的 open/default_open/on_open_change 语义；
  - 基于 OverlayManager 的定位与 z-index 分配；
  - 部分关闭逻辑复用 `tooltip::update_open_state`。
- 差异：
  - Tooltip 更适合单行短文案；Popover 适合承载复杂布局和操作；
  - Tooltip 默认 trigger 为 `Hover`，Popover 默认 trigger 为 `Click`；
  - 样式上 Tooltip 是深色小气泡，Popover 是浅色卡片。

---

## Popconfirm：危险操作确认（MVP）

> 实现位置：`src/components/popconfirm.rs`  
> 示例：`examples/popconfirm_demo.rs`

### 设计目标

- 针对“删除/停用”等危险操作提供二次确认弹层：
  - 提供固定的标题 + 描述 + 确认/取消按钮布局；
  - 默认使用 `Click` 触发，与 Popover 保持一致；
  - 点击确认/取消后自动关闭，并触发业务回调。
- 复用 Popover 的浮层基础设施：
  - Popconfirm 自身不直接与 OverlayManager 交互，而是将内容包裹在 `Popover` 中；
  - Popconfirm 负责管理受控/非受控 open 状态；
  - Popover 负责处理 trigger 行为和浮层定位。

### PopconfirmProps 核心字段

> 类型定义：`src/components/popconfirm.rs`

字段 | 类型 | 说明
--- | --- | ---
`title` | `String` | 主标题文案，必填。
`description` | `Option<String>` | 说明性文本，可选。
`ok_text` | `Option<String>` | 确认按钮文案，默认 `"确定"`。
`cancel_text` | `Option<String>` | 取消按钮文案，默认 `"取消"`。
`on_confirm` | `Option<EventHandler<()>>` | 点击确认时触发。
`on_cancel` | `Option<EventHandler<()>>` | 点击取消时触发。
`ok_type` | `Option<ButtonType>` | 确认按钮类型，默认 `Primary`。
`ok_danger` | `bool` | 确认按钮是否使用危险样式。
`placement` | `Option<TooltipPlacement>` | 浮层位置，默认 `Top`。
`trigger` | `TooltipTrigger` | 触发方式，默认 `Click`。
`open` | `Option<bool>` | 受控 open 状态。
`default_open` | `Option<bool>` | 非受控初始 open 状态。
`on_open_change` | `Option<EventHandler<bool>>` | 打开状态变更回调。
`disabled` | `bool` | 是否禁用交互。
`class` / `overlay_class` / `overlay_style` | `Option<String>` | 触发区与浮层的类名/样式。
`children` | `Element` | 触发元素（一般是危险按钮）。

### 行为与状态管理

- Popconfirm 内部维护 `open_state: Signal<bool>`，并根据是否传入 `open` 决定：
  - 非受控：`open_state` 作为唯一来源；点击触发区/点击空白/点击按钮/按 Esc 都会更新该状态；
  - 受控：`open` 作为唯一来源；所有交互只通过 `on_open_change(next)` 通知父组件。
- 与 Popover 的关系：
  - Popconfirm 使用受控模式驱动内部 `Popover`：
    - 始终向 Popover 传入 `open: Some(current_open)`；
    - 将 Popover 的 `on_open_change` 映射到自身的 `update_open_state`（与 Tooltip/Popover 复用同一辅助函数）。
  - 点击确认/取消按钮时：
    - 先调用对应的 `on_confirm` / `on_cancel` 回调；
    - 再调用 `update_open_state(..., false)` 关闭 Popconfirm（受控模式下仅触发回调，由上一层修改 `open`）。

### 布局与样式

> 样式定义：`src/theme.rs` 中的 `.adui-popconfirm-*`

结构类名：

- `.adui-popconfirm-inner`：Popconfirm 内部整体容器；
- `.adui-popconfirm-body`：标题与描述区域；
- `.adui-popconfirm-title`：标题行；
- `.adui-popconfirm-description`：描述文案；
- `.adui-popconfirm-footer`：底部按钮区域，右对齐排列确认/取消按钮。

默认样式使 Popconfirm 看起来像一个有固定结构的 Popover 卡片，按钮间距与 Button 样式保持一致。

### 示例：危险删除确认

摘自 `examples/popconfirm_demo.rs`：

```rust
let api = use_message();

Popconfirm {
    title: "确定要删除这条记录吗？".to_string(),
    description: Some("删除后无法恢复，请谨慎操作。".to_string()),
    ok_text: Some("删除".to_string()),
    cancel_text: Some("取消".to_string()),
    ok_type: Some(ButtonType::Primary),
    ok_danger: true,
    on_confirm: move |_| {
        if let Some(msg) = api.clone() {
            msg.success("已删除");
        }
    },
    on_cancel: move |_| {
        if let Some(msg) = api.clone() {
            msg.info("已取消删除");
        }
    },
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            danger: true,
            "删除"
        }
    },
}
```

该示例演示了 Popconfirm 与 `use_message` 的协同使用：确认/取消操作分别弹出不同的全局消息。

---

## Dropdown：菜单型下拉（MVP）

> 实现位置：`src/components/dropdown.rs`  \
> 示例：`examples/dropdown_demo.rs`

### 设计目标

- 为按钮/链接提供轻量的“操作菜单”下拉：
  - 支持点击触发、hover 触发；
  - 支持左对齐或右对齐展示菜单；
  - 对菜单项点击进行回调，并在点击后关闭菜单。
- 基于统一的 OverlayInfra：
  - 使用 `OverlayKind::Dropdown` 和 `use_floating_layer` 管理 z-index；
  - 非受控 + click 模式下通过 `use_floating_close_handle` 支持点击空白关闭；
  - Esc 键关闭行为与 Tooltip/Popover 统一。

### DropdownProps 与 DropdownItem

> 类型定义：`src/components/dropdown.rs`

`DropdownItem`:

字段 | 类型 | 说明
--- | --- | ---
`key` | `String` | 唯一标识，`on_click` 回调中会返回。
`label` | `String` | 展示文案。
`disabled` | `bool` | 是否禁用该项。

`DropdownProps` 主要字段：

字段 | 类型 | 说明
--- | --- | ---
`items` | `Vec<DropdownItem>` | 菜单项列表。
`trigger` | `DropdownTrigger` | 触发方式，`Click`（默认）或 `Hover`。
`placement` | `Option<DropdownPlacement>` | `BottomLeft` / `BottomRight`，默认 `BottomLeft`。
`open` | `Option<bool>` | 受控 open 状态。
`default_open` | `Option<bool>` | 非受控初始 open 状态。
`on_open_change` | `Option<EventHandler<bool>>` | 打开状态变更回调。
`on_click` | `Option<EventHandler<String>>` | 点击菜单项时触发，参数为项的 `key`。
`disabled` | `bool` | 是否禁用整体 Dropdown。
`class` | `Option<String>` | 触发区 wrapper 的类名。
`overlay_class` / `overlay_style` | `Option<String>` | 浮层菜单类名/样式。
`overlay_width` | `Option<f32>` | 可选的菜单最小宽度。
`children` | `Element` | 触发元素（Button/链接/图标等）。

### 行为与状态管理

- 与 Tooltip/Popover/Popconfirm 相同的受控/非受控语义：
  - 非受控：使用内部 `open_state: Signal<bool>`；
  - 受控：`open` 为真源，内部只通过 `on_open_change` 通知。
- 触发方式：
  - `Click`：
    - 点击触发区切换 open；
    - 使用 `use_floating_close_handle`：
      - 内部点击（触发区/菜单）先调用 `mark_internal_click()`，不会触发关闭；
      - 点击空白关闭菜单（非受控模式）。
  - `Hover`：
    - `mouseenter` 打开，`mouseleave` 关闭；
    - Hover 模式下不使用点击空白关闭。
- 菜单项点击：
  - 若项未禁用：
    - 先通过 `update_open_state(..., false)` 关闭菜单；
    - 再调用 `on_click(key)` 回调。

### 布局与样式

> 样式定义：`src/theme.rs` 中 `.adui-dropdown-*`

结构类名：

- `.adui-dropdown-root`：触发区外层容器；
- `.adui-dropdown-menu`：菜单浮层容器；
- `.adui-dropdown-menu-list`：`<ul>` 列表；
- `.adui-dropdown-menu-item`：单个菜单项；
- `.adui-dropdown-menu-item-disabled`：禁用菜单项；
- `.adui-dropdown-sm` / `.adui-dropdown-lg`：跟随 ConfigProvider 的尺寸调整 item 高度和字体大小。

定位：

- 内联样式控制浮层：`top: 100%; margin-top: 4px; z-index: ...`；
- 水平位置：
  - `BottomLeft`：`left: 0;`
  - `BottomRight`：`right: 0;`

### 示例：基础 Dropdown 菜单

摘自 `examples/dropdown_demo.rs`：

```rust
fn default_items() -> Vec<DropdownItem> {
    vec![
        DropdownItem::new("new", "新建文档"),
        DropdownItem::new("open", "打开..."),
        DropdownItem::new("share", "分享"),
        DropdownItem { key: "disabled".into(), label: "禁用项".into(), disabled: true },
    ]
}

let api = use_message();
let mut last_click = use_signal(|| String::new());

Dropdown {
    items: default_items(),
    trigger: DropdownTrigger::Click,
    placement: Some(DropdownPlacement::BottomLeft),
    on_click: move |key: String| {
        last_click.set(key.clone());
        if let Some(msg) = api.clone() {
            msg.info(format!("选择菜单项: {key}"));
        }
    },
    children: rsx! {
        Button {
            r#type: ButtonType::Default,
            "基础下拉菜单"
        }
    },
}
```

此外示例还展示了 `BottomRight` 右对齐菜单与 `Hover` 触发下拉菜单的用法。
