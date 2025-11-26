# Form 使用说明

`Form` 组件仿照 Ant Design 6.x 的 API，将布局、校验与上下文接口封装为纯 Rust/Dioxus 范式。整体由三部分组成：

- `use_form` / `FormHandle`：创建或复用表单实例。
- `Form`：负责提供上下文、渲染整体布局、处理提交。
- `FormItem` + `use_form_item_control`：包装具体控件并连接到表单存储。

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
`reset_fields()` | 清空所有值与错误，并通知已注册的字段 scope 重渲染。

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

## 校验规则

`FormRule` 支持以下字段：

- `required: bool`：必填判断（对字符串/数组/对象会进行空值检查）。
- `min` / `max`：既可以限制长度也可以限制数值，内部自动区分字符串/数字。
- `len`：固定长度，用于验证码等场景。
- `pattern: Option<String>`：正则表达式字符串，编译失败会在错误消息中提示。
- `validator: Option<FormValidator>`：自定义函数签名 `fn(Option<&Value>) -> Result<(), String>`，可用于复杂逻辑。
- `message: Option<String>`：优先级最高的自定义提示语。

校验触发时机会包括：

1. `FormItem` 挂载或 `rules` 变更。
2. `FormItemControlContext::set_value`（即 oninput/onchange）调用。
3. `Form` 提交前的 `validate_all`。

当前版本仅实现同步校验，异步/`validate_trigger` 将在后续迭代。

## 布局与样式

- 全局样式由 `theme.rs` 中的 `FORM_STYLE` 定义，包含 `.adui-form-*`、`.adui-form-item-*` 等类。
- `Horizontal` 布局：label 固定宽度（默认 120px），右对齐。`Vertical/Inline` 会将 label 换行并靠左排列。
- `RequiredMark::Default` + `rules` 中含 `required` 时会展示红色星号；`colon` 为 `true` 时在 label 末尾渲染冒号。
- 错误状态会自动附加 `.adui-form-item-has-error`，默认样式会给首个控件添加红色边框与阴影。

## 示例

`examples/form_demo.rs` 展示了典型用法：

```rust
#[component]
fn FormDemo() -> Element {
    let form_handle = use_signal(use_form);
    rsx! {
        Form {
            form: Some(form_handle.read().clone()),
            on_finish: move |evt| log::info!("ok: {:?}", evt.values),
            on_finish_failed: move |err| log::warn!("fail: {:?}", err.errors),
            FormItem {
                name: Some("username".into()),
                label: Some("用户名".into()),
                rules: Some(vec![FormRule {
                    required: true,
                    message: Some("请输入用户名".into()),
                    ..FormRule::default()
                }]),
                FormTextInput {}
            }
            button { r#type: \"submit\", \"提交\" }
            Button {
                r#type: ButtonType::Text,
                onclick: {
                    let form = form_handle.clone();
                    move |_| form.read().reset_fields();
                },
                \"重置\"
            }
        }
    }
}
```

运行 `dx serve --example form_demo` 即可查看效果。示例中使用 `ThemeProvider` 包裹整个 App，可直接复用项目主题变量。

## 与 Ant Design 的差异

- 目前只支持 `String` 名称（AntD 支持数组路径），适合简单表单；后续计划扩展 `Vec<NameSegment>`。
- 尚未提供 `Form.List`、`dependencies`、`validateTrigger` 等高级功能。
- 校验均为同步，`async validator` 与 scroll-to-error 仍在规划中。

尽管存在差异，核心 API（`useForm`、`Form.Item`、规则字段）与布局体验已与 AntD 6.0 接近，适合在 Dioxus 项目中构建常规表单。若需要自定义控件，只需在组件中调用 `use_form_item_control()` 并在 `oninput/onchange` 内写入即可。
