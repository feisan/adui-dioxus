# Checkbox / Radio / Switch 使用说明

本文件汇总了经典表单布尔控件与单选控件：`Checkbox` / `CheckboxGroup`、`Radio` / `RadioGroup` / `RadioButton`、`Switch`，并说明它们与 `Form` 的集成方式。

## Checkbox / CheckboxGroup

### CheckboxProps 说明

> 定义位置：`src/components/checkbox.rs:14` 起。

字段 | 类型 | 说明
--- | --- | ---
`checked` | `Option<bool>` | 受控勾选状态；存在时不使用内部 `default_checked`。
`default_checked` | `bool` | 非受控初始状态，默认 `false`。
`indeterminate` | `bool` | 不确定态，仅影响视觉表现（常用于“全选”半选中）。
`disabled` | `bool` | 是否禁用。
`value` | `Option<String>` | 在 `CheckboxGroup` 中使用的值，与选中集合对应。
`status` | `Option<ControlStatus>` | 状态样式：`success`/`warning`/`error`。
`class` / `style` | `Option<String>` | 自定义类名与样式。
`on_change` | `Option<EventHandler<bool>>` | 勾选状态变更时回调（仅在非 Group 模式调用）。
`children` | `Element` | 复选框右侧的 label 内容。

视觉样式主要由以下类控制（见 `src/theme.rs`）：

- `.adui-checkbox` 根节点（`label`），内部包含隐藏的 `<input>` 与 `.adui-checkbox-inner`。
- `.adui-checkbox-checked` / `.adui-checkbox-indeterminate` / `.adui-checkbox-disabled` 控制不同状态。

### CheckboxGroupProps 说明

字段 | 类型 | 说明
--- | --- | ---
`value` | `Option<Vec<String>>` | 受控模式下的选中值集合。
`default_value` | `Vec<String>` | 非受控初始选中集合，默认空。
`disabled` | `bool` | 整组禁用。
`class` / `style` | `Option<String>` | `.adui-checkbox-group` 容器样式。
`on_change` | `Option<EventHandler<Vec<String>>>` | 选中集合变更时回调。
`children` | `Element` | 内部 `Checkbox` 子节点。

### Group 行为与 Form 集成

- Group 内部通过 `CheckboxGroupContext` 使用 `Signal<Vec<String>>` 管理当前选中集合，并暴露给子 `Checkbox`。
- 勾选逻辑：
  - 若某个 `value` 已在集合中 → 移除；否则 → 插入。
  - 变更后：
    - 调用 `on_change(Vec<String>)`（若存在）；
    - **若绑定到 `FormItem`**，会将集合写回 `FormHandle` 对应字段，类型为：
      - `Value::Array([Value::String("rust"), ...])`；
    - 在非受控模式时，更新内部 `Signal<Vec<String>>` 以驱动 UI。
  - 受控模式下（传入 `value`），仅通过 `on_change` 通知外部，内部不会持久化集合。

### 示例：多选标签

摘自 `examples/input_checkbox_demo.rs`：

```rust
FormItem {
    name: Some("tags".into()),
    label: Some("标签".into()),
    CheckboxGroup {
        default_value: vec!["rust".into()],
        Checkbox { value: Some("rust".into()), "Rust" }
        Checkbox { value: Some("dioxus".into()), "Dioxus" }
        Checkbox { value: Some("web".into()), "Web" }
    }
}
```

- 在 `FormItem` 中使用时，不需要手动操作 `FormHandle`，`CheckboxGroup` 会自动将选中集合同步到 `tags` 字段。
- 提交表单时，`tags` 字段是一个字符串数组，对应 `Value::Array([...])`。


## Radio / RadioGroup / RadioButton

### RadioProps 说明

> 定义位置：`src/components/radio.rs:17` 起。

字段 | 类型 | 说明
--- | --- | ---
`value` | `String` | 当前 Radio 的值，用于 Group 场景。
`checked` | `Option<bool>` | 独立使用时的受控勾选状态。
`default_checked` | `bool` | 独立模式下的非受控初始勾选状态。
`disabled` | `bool` | 禁用。
`status` | `Option<ControlStatus>` | 状态样式。
`class` / `style` | `Option<String>` | 样式。
`on_change` | `Option<EventHandler<String>>` | 选中时回调，参数为当前 Radio 的 `value`。
`children` | `Element` | label 内容。
`button` | `bool` | 是否渲染为按钮样式 Radio（`RadioButton` 内部即设置此项）。

### RadioGroupProps 说明

字段 | 类型 | 说明
--- | --- | ---
`value` | `Option<String>` | 受控选中值。
`default_value` | `Option<String>` | 非受控初始选中值。
`name` | `Option<String>` | 原生 radio 的 name 属性，便于浏览器层面的互斥行为。
`disabled` | `bool` | 整组禁用。
`class` / `style` | `Option<String>` | 容器样式。
`on_change` | `Option<EventHandler<String>>` | 选中值变更时回调。
`children` | `Element` | 内部 `Radio` 子节点。

### Group 行为与 Form 集成

- Group 通过 `RadioGroupContext` 使用 `Signal<Option<String>>` 记录当前选中值。
- 初始化选中值优先级：
  1. 外部 `value`（受控）。
  2. Form 中当前字段值（通过 `form_radio_value_to_string` 从 `Value` 转换）。
  3. `default_value`（非受控）。
- 点击某个 `Radio` 时：
  - 若存在 Form 绑定，则 `FormItemControlContext::set_string` 写回当前值；
  - 调用 Group `on_change`；
  - 非受控模式下更新内部 `Signal<Option<String>>`。

### RadioButton

- `RadioButton` 只是一个方便包装：

```rust
#[component]
pub fn RadioButton(props: RadioProps) -> Element {
    Radio(RadioProps { button: true, ..props })
}
```

- 在样式上使用 `.adui-radio-button` 类，与 `.adui-radio` 有不同的视觉表现（可在后续样式中按需增强）。


## Switch

`Switch` 是布尔值开关，行为对齐 Ant Design：点击或按 Enter/Space 切换状态，并提供 `role="switch"` 与 `aria-checked`。

### SwitchProps 说明

> 定义位置：`src/components/switch.rs:10` 起。

字段 | 类型 | 说明
--- | --- | ---
`checked` | `Option<bool>` | 受控状态。
`default_checked` | `bool` | 非受控初始状态，默认 `false`。
`disabled` | `bool` | 禁用。
`size` | `SwitchSize` (`Default/Small`) | 尺寸，对应不同宽高与 handle 大小。
`checked_children` | `Option<Element>` | 打开时的内容（例如“开”）。
`un_checked_children` | `Option<Element>` | 关闭时的内容（例如“关”）。
`status` | `Option<ControlStatus>` | 状态样式。
`class` / `style` | `Option<String>` | 样式。
`on_change` | `Option<EventHandler<bool>>` | 切换后回调，参数为新的布尔状态。

### 行为与 Form 集成

- 在存在 `FormItemControlContext` 时：
  - 当前状态优先从表单字段值读取：若 `Value::Bool(true)` → 视为已开启；否则视为关闭。
  - 每次切换都会调用 `ctx.set_value(Value::Bool(next))` 写回表单，并触发当前字段的校验。
- 在不使用 Form 时：
  - 受控模式：只调用 `on_change(next)`，不写内部状态。
  - 非受控模式：使用 `Signal<bool>` 存储当前状态，初始值来自 `default_checked`。

### 示例：Form 中的布尔开关

摘自 `examples/input_checkbox_demo.rs`：

```rust
FormItem {
    name: Some("newsletter".into()),
    label: Some("订阅邮件".into()),
    Switch {
        default_checked: true,
        checked_children: Some(rsx!("开")),
        un_checked_children: Some(rsx!("关")),
    }
}
```

- 用户点击或按 Enter/Space 时，Switch 会切换状态，并将 `newsletter` 字段更新为 `true/false`。
- 与 Checkbox / Radio 一样，Reset 时通过 `form_handle.read().reset_fields()` 清空字段值，控件会回到初始状态（由 `default_checked` 控制）。


### 关于 `default_checked` / `default_value` 与 Form 初始值

- 在 **不使用 Form** 时：
  - `default_checked` / `default_value` 用于初始化内部 `Signal`（单 Checkbox/Switch 或 CheckboxGroup），后续状态完全由内部 state 或受控 `checked/value` 决定。
- 在 **`FormItem` 场景** 中：
  - 推荐通过 `Form` 的 `initial_values` 或 `FormHandle::set_field_value` 初始化布尔/数组/字符串字段；
  - 单个 `Checkbox` / `CheckboxGroup` 在存在 `FormItemControlContext` 时，以 `FormStore` 中的值为唯一真相源，`default_checked` / `default_value` 不再参与后续渲染；
  - `Switch` 在 Form 中仍会使用 `default_checked` 作为“无值时的 UI 初始状态”，但真正的字段值以 `FormStore` 为准（点击切换后写入 `Value::Bool`，`reset_fields()` 会清空字段值并使 UI 回到 `default_checked` 所对应的视觉状态）；
  - 简而言之：**Form 中的 Checkbox/Radio/Switch 的真实值存放在 FormStore 中，控件负责渲染和写回，内部 state 仅作为 UI 过渡。**

## 小结与建议

- 简单使用场景：
  - 推荐始终在 `FormItem` 中包裹控件，让控件自动与字段绑定，避免手动管理 `FormHandle`。
- 受控高级用法：
  - 若需要从 URL 或全局状态中驱动选中状态，可以传入 `value/checked` 并监听 `on_change`，再通过外部 `set_field_value` 驱动 Form。
- 可访问性：
  - Checkbox 与 Radio 使用 `<label>` 包裹隐藏的 `<input>`，可点击 label 区域；Switch 使用 `<button role="switch">`，同时设置 `aria-checked` / `aria-disabled`，便于屏幕阅读器识别。
