# Form 使用说明

`Form` 组件仿照 Ant Design 6.x 的 API，将布局、校验与上下文接口封装为纯 Rust/Dioxus 范式。整体由三部分组成：

- `use_form` / `FormHandle`：创建或复用表单实例。
- `Form`：负责提供上下文、渲染整体布局、处理提交。
- `FormItem` + `use_form_item_control`：包装具体控件并连接到表单存储。

> 选择器类控件（Select / TreeSelect / Cascader / AutoComplete）与 Form 的集成说明，见 `docs/select_family.md`，下面仅对 Form 本身行为进行说明。

## 设计要点（重要）

- **单一数据源**：字段值与错误全部存放在 `FormStore`（`FormHandle` 内部），所有在 `FormItem` 中的控件通过 `FormItemControlContext` 读取/写回，控件自身的 `default_*` 只在不使用 Form 时生效。
- **规则与值分离**：校验规则存放在 `FormContext.registry` 中，与 `FormStore` 分离；`reset_fields()` 只清空值与错误，不会清理 registry 中的 `FormRule`，因此 `required` 等规则在重置后依然生效。
- **稳定的 registry 引用**：`Form` 在首次渲染时创建一次 `Rc<RefCell<HashMap<String, Vec<FormRule>>>>`，整个生命周期内复用同一份引用，保证 `FormItem` 注册的规则与 `Form` 提交时使用的规则始终一致。
- **提交校验策略**：
  - `Form` 在 `onsubmit` 中调用内部的 `validate_all`。
  - 若 `values()` 为空且存在至少一个 `required` 规则，会对所有必填字段执行一次校验并直接视为失败，覆盖“首次提交”和“重置后立刻提交”的场景。
  - 否则按字段名逐一执行规则校验，根据是否存在错误决定触发 `on_finish` 还是 `on_finish_failed`。
- **布尔字段的 `required` 语义**：内部的 `value_is_empty` 会将 `Value::Bool(false)` 与 `None` 同样视为“空”，因此在 Checkbox / Switch 场景下，只需配置 `required: true` 即可表示“必须为 true 才视为已填写”。

## FormHandle / use_form

```rust
let form = use_signal(use_form);
let values = form.read().values(); // HashMap<String, Value>
form.read().set_field_value("username", Value::String("lex".into()));
form.read().reset_fields();
```

`FormHandle` 当前暴露的常用方法：

名称 | 说明
--- | ---
`set_field_value(name, Value)` | 写入字段值并触发对应 `FormItem` 重渲染。
`get_field_value(name)` | 读取单个字段。
`set_error(name, Option<String>)` / `get_error(name)` | 手动控制错误信息。
`values()` / `errors()` | 复制整个缓存。
`reset_fields()` | 清空所有值与错误，并通知已注册的字段 scope 重渲染（不会清理校验规则）。

> 注意：`FormHandle` 通过内部的 `ScopeId` 列表触发渲染，不再依赖跨作用域信号，因此可以安全地在父组件回调里调用 `reset_fields()` 等方法。

## FormProps 重点

属性 | 类型 | 描述
--- | --- | ---
`layout` | `FormLayout` (`Horizontal/Vertical/Inline`) | 控制 label 与 control 的排列方式，默认水平。
`size` | `ControlSize` (`Small/Middle/Large`) | 同步 label 高度与控件尺寸，影响 `.adui-form-*` 样式。
`colon` | `bool` | 是否为 label 自动追加冒号，默认 `false`。
`required_mark` | `RequiredMark` (`None/Optional/Default`) | 控制必填星号的展示策略。
`label_align` | `LabelAlign` (`Left/Right`) | `Horizontal` 布局下 label 对齐方式。
`label_col` / `wrapper_col` | `Option<ColProps>` | 预留与 Grid 结合用的栅格配置（MVP 阶段存储但未渲染具体栅格元素）。
`disabled` | `bool` | 为所有 `FormItem` 控制上下文的 `disabled`。
`initial_values` | `Option<HashMap<String, Value>>` | 首次渲染时批量写入字段值。
`form` | `Option<FormHandle>` | 受控模式，可以传入 `use_form()` 的结果；不传则内部创建。
`on_finish` / `on_finish_failed` | `EventHandler<FormFinishEvent/FailedEvent>` | 分别在校验通过或失败时触发，事件中带出 `values` 或 `errors`。

`Form` 会在 `onsubmit` 时调用 `validate_all`，因此自定义按钮只需设置 `type="submit"` 即可复用默认行为。

## FormItemProps 重点

属性 | 类型 | 描述
--- | --- | ---
`name` | `Option<String>` | 表单字段名。省略时仅渲染 label/help，不与 store 绑定。
`label` / `tooltip` / `extra` / `help` | `Option<String>` | 文案展示，与 AntD API 一致。
`rules` | `Option<Vec<FormRule>>` | 同步规则（见下节）。`FormItem` 挂载时会自动注册并初始化校验。
`class` / `style` | `Option<String>` | 包裹 `.adui-form-item` 的自定义样式。
`has_feedback` | `bool` | 在无错误时也绘制反馈图标占位，方便与后续 icon 结合。

`FormItem` 内部通过 `use_context_provider` 暴露 `FormItemControlContext`，自定义控件可使用 `use_form_item_control()` 读取当前字段：

```rust
#[component]
fn FormTextInput() -> Element {
    let control = use_form_item_control();
    let value = control
        .as_ref()
        .and_then(|ctrl| ctrl.value())
        .and_then(|val| val.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    rsx! {
        input {
            class: "adui-input",
            value: "{value}",
            disabled: control.as_ref().is_some_and(|c| c.is_disabled()),
            oninput: move |evt| {
                if let Some(ctrl) = control.as_ref() {
                    ctrl.set_string(evt.value());
                }
            }
        }
    }
}
```

## 与 DatePicker / TimePicker / RangePicker 集成

日期时间组件在 Form 中被视为普通字段，内部会将选中的值写入 `FormStore`，并参与校验与重置：

- `DatePicker` / `RangePicker`：\n  - 默认格式为 `YYYY-MM-DD`；\n  - 在 Form 中与 `FormRule` 一起使用时，可以直接配置 `required: true` 或自定义 `validator` 实现「开始日期早于结束日期」等规则；\n  - 调用 `FormHandle::reset_fields()` 时，会同步清空/恢复所有日期字段的 UI 与内部值。\n- `TimePicker`：\n  - 默认格式为 `HH:mm:ss`；\n  - 可与 `DatePicker` 组合成「日期 + 时间」的复杂表单项，通过规则控制时间范围。

示例（摘自日期时间 demo 的简化版本）：

```rust
let form = use_signal(use_form);

Form {
    form: Some(form.read().clone()),
    FormItem {
        name: Some("start_date".into()),
        label: Some("开始日期".into()),
        rules: Some(vec![FormRule {
            required: true,
            message: Some("请选择开始日期".into()),
            ..FormRule::default()
        }]),
        DatePicker {}
    }
    FormItem {
        name: Some("range".into()),
        label: Some("日期区间".into()),
        RangePicker {}
    }
    FormItem {
        name: Some("appointment_time".into()),
        label: Some("预约时间".into()),
        TimePicker {}
    }
}
```

> 建议在 Form 场景下仅通过 `FormItem.name` 管理字段值，不再单独传入 `value`/`default_value`，避免受控/非受控状态冲突；重置与提交逻辑统一通过 `FormHandle` 控制。

（其余 Form 校验、布局、FormList 等小节保持原有内容，未做结构性修改，仅在顶部增加了对选择器家族文档的链接。）
