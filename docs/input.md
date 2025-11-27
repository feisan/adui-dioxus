# Input / TextArea 使用说明

`Input` 与 `TextArea` 是 Ant Design 风格的文本输入组件，封装了受控/非受控模式、清空按钮、前后缀图标，以及与 `FormItem` 的集成逻辑。

## 核心能力概览

- **受控 / 非受控**：
  - 受控：传入 `value: Option<String>`，组件不会维护内部状态，只在变更时触发 `on_change`，外部负责更新。
  - 非受控：不传 `value`，可选传入 `default_value` 作为初始值，内部通过 `Signal<String>` 保存当前内容。
  - 在 `FormItem` 中：若存在 `FormItemControlContext`，则优先从表单 store 读取与写入值（会覆盖内部状态）。
- **清空按钮与前后缀**：
  - `prefix` / `suffix`：在输入框左右侧渲染任意 RSX 内容（图标、单位、按钮等），整体使用 `.adui-input-affix-wrapper` 包裹。
  - `allow_clear`：当内容非空且未禁用时，显示一个清空按钮，点击会将值置空，并同步表单/回调。
- **状态与禁用**：
  - `status: Option<ControlStatus>`：附加状态类（`success` / `warning` / `error`），与 Form 校验态对齐。
  - `disabled: bool`：禁用输入，表单上下文的 `disabled` 也会生效。
- **键盘行为**：
  - `Input` 支持 `on_press_enter`，在按下 Enter 时触发，常用于搜索或提交场景。
  - `TextArea` 当前只处理 `on_change`，不额外拦截键盘事件。

## InputProps 说明

> 定义位置：`src/components/input.rs:10` 起。

主要字段：

字段 | 类型 | 说明
--- | --- | ---
`value` | `Option<String>` | 受控值；存在时组件不写内部状态。
`default_value` | `Option<String>` | 非受控初始值，仅在首次渲染时生效。
`placeholder` | `Option<String>` | 占位文字。
`disabled` | `bool` | 是否禁用，禁用时同时阻止内部更新与回调。
`status` | `Option<ControlStatus>` | 视觉状态：`Success/Warning/Error`。
`prefix` / `suffix` | `Option<Element>` | 左右侧附加内容，渲染在 `.adui-input-affix-wrapper` 中。
`allow_clear` | `bool` | 内容非空时显示清空按钮。
`class` / `style` | `Option<String>` | 自定义类名和行内样式。
`on_change` | `Option<EventHandler<String>>` | 内容变更时回调，参数为最新字符串。
`on_press_enter` | `Option<EventHandler<()>>` | 按下 Enter 时回调，仅 `Input` 支持。

### 受控 / 非受控 / Form 集成逻辑

内部通过三个来源决定当前值（优先级从高到低）：

1. 若存在 `FormItemControlContext`（即包裹在 `FormItem` 中）：
   - 从 `ctx.value()` 读取 `serde_json::Value`，通过 `form_value_to_string` 转成字符串：
     - `String` → 直接使用；
     - `Number` / `Bool` → 调用 `to_string`；
     - 其他类型 → 视为空字符串。
   - 在 `oninput` 中调用 `ctx.set_string(next)` 写回表单 store，并触发当前字段校验。
2. 若外部传入 `value`（受控模式）：
   - 直接渲染该值，忽略内部 `Signal`。
   - 变更时仅调用 `on_change`，不修改内部 state。
3. 其他情况（非受控）：
   - 使用 `default_value` 初始化内部 `Signal<String>`，后续变更只写入 `Signal`。

> 简化后行为：**在 Form 中使用时，不必手动传 `value`，Input 会自动与当前字段绑定并驱动校验。**

### 示例：在 Form 中使用 Input

摘自 `examples/form_demo.rs`：

```rust
FormDemo() -> Element {
    let form_handle = use_signal(use_form);

    rsx! {
        Form {
            form: Some(form_handle.read().clone()),
            on_finish: move |evt| {
                // evt.values: HashMap<String, Value>
            },
            FormItem {
                name: Some("username".into()),
                label: Some("用户名".into()),
                rules: Some(vec![FormRule {
                    required: true,
                    message: Some("请输入用户名".into()),
                    ..FormRule::default()
                }]),
                Input {
                    placeholder: Some("请输入用户名".into()),
                }
            }
        }
    }
}
```

- 未传 `value`，Input 将自动从 `FormItem` 绑定的字段 `username` 读取/写入。
- 校验失败时，`FormItem` 会展示错误提示，Input 也会通过 `.adui-form-item-has-error` 的样式得到红色边框和阴影。

## TextAreaProps 说明

字段 | 类型 | 说明
--- | --- | ---
`value` / `default_value` | `Option<String>` | 与 `Input` 相同的受控/非受控语义。
`placeholder` | `Option<String>` | 占位文本。
`rows` | `Option<u16>` | 文本区默认行数，默认值为 3。
`disabled` | `bool` | 是否禁用。
`status` | `Option<ControlStatus>` | 同步 Form 状态样式。
`class` / `style` | `Option<String>` | 自定义样式。
`on_change` | `Option<EventHandler<String>>` | 内容变更回调。

内部实现与 `Input` 共用 `resolve_current_value` / `apply_input_value` 逻辑，只是渲染为 `<textarea>` 并附加类名：

- `.adui-input` + `.adui-input-textarea`：控制文本区的边框、最小高度与 `resize` 行为。

### 示例：多行文本与表单

摘自 `examples/input_checkbox_demo.rs`：

```rust
FormItem {
    name: Some("bio".into()),
    label: Some("简介".into()),
    TextArea {
        rows: Some(3),
        placeholder: Some("简单介绍一下自己".into()),
    }
}
```

提交表单时，`bio` 字段会出现在 `FormFinishEvent.values` 中，对应 `Value::String` 类型。

### 关于 `default_value` 与 Form 初始值

- 在 **不使用 Form** 时：
  - `default_value` 仅在首次渲染时用于初始化内部 `Signal<String>`，之后控件完全由内部状态或受控 `value` 驱动。
- 在 **`FormItem` 场景** 中：
  - 推荐使用 `Form` 的 `initial_values` 或 `FormHandle::set_field_value` 设置初始值；
  - 一旦存在 `FormItemControlContext`，Input/TextArea 会**完全忽略**自身的 `default_value`，始终以 `FormStore` 为唯一真相源（通过 `ctx.value()` 读取，通过 `ctx.set_string` 写回）；
  - 调用 `FormHandle::reset_fields()` 时会清空字段值与错误，Input/TextArea 会立即渲染为空字符串，与当前 `FormStore` 状态保持一致。

## 与 Ant Design 的差异与注意事项

- 当前版本未实现 `Input.Password` / `Input.Search` / `InputNumber` / `Select`，仅提供基础 `Input` / `TextArea` 能力；扩展能力会在后续计划中补齐（见 `plan/0003.md`）。
- `allow_clear` 的清空按钮目前使用简单的 "×" 字符作为图标，后续可以替换为统一的 `Icon` 组件。
- `addonBefore/After` 等组合输入区域暂未暴露为 props，但通过 `Flex` / `Space` 可以在布局层实现类似效果。
- 在 Form 外使用时，若业务本身有受控状态管理，建议优先使用 `value + on_change` 模式，避免内部 state 与外部 state 不一致。
