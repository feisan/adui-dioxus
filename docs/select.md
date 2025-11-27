# Select 使用说明

`Select` 是 Ant Design 风格的下拉选择组件，支持单选、多选、搜索及与 `FormItem` 的集成。当前版本实现了核心表单场景所需的 MVP 能力：

- 单选 / 多选（`multiple`）
- 简单搜索（客户端 label 包含过滤，`show_search`）
- 受控 / 非受控模式
- 与 `Form` 的值与校验集成

> 说明：TreeSelect / Cascader / AutoComplete 将在同一计划（`plan/0007.md`）中陆续补充，本节集中介绍基础 `Select`。

## 核心能力概览

- **选项建模**：
  - `SelectOption`：`{ key: String, label: String, disabled: bool }`，用于描述一个选项；
  - 内部统一使用 `key: String` 作为值，便于与 `serde_json::Value` 与 FormStore 做互转。
- **单选 / 多选**：
  - 单选：`multiple = false`（默认），通过 `value: Option<String>` 或 Form 字段值驱动；
  - 多选：`multiple = true`，通过 `values: Option<Vec<String>>` 或 Form 字段值驱动；
- **搜索过滤**：
  - `show_search = true` 时，弹层顶部会出现一个搜索输入框；
  - 当前实现为“label 包含、大小写不敏感”的简单过滤，适合中小规模本地数据；
- **受控 / 非受控 / Form 集成**：
  - 受控：传入 `value` / `values`，组件不维护内部选中状态，只在变更时触发 `on_change`，由外部更新；
  - 非受控：不传 `value` / `values`，组件内部维护选中集合；
  - Form 场景：若存在 `FormItemControlContext`，则以 FormStore 为唯一真相源，读写都走 Form。
- **关闭行为**：
  - 单选：选择某项后自动关闭弹层；
  - 多选：选择/取消选项后保持打开，便于连续选择；
  - Esc 键：在弹层打开时按 Esc 关闭；
  - 点击空白：点击 Select 外部区域时关闭弹层（使用 document-level click 检测）。

## SelectProps 说明

> 定义位置：`src/components/select.rs:22` 起。

主要字段：

字段 | 类型 | 说明
--- | --- | ---
`value` | `Option<String>` | 单选时的受控值；存在时组件不写内部 state。
`values` | `Option<Vec<String>>` | 多选时的受控值集合；通常与 `multiple = true` 搭配使用。
`options` | `Vec<SelectOption>` | 选项列表，每个选项包含 `key`/`label`/`disabled`。
`multiple` | `bool` | 是否启用多选模式，默认 `false`。
`allow_clear` | `bool` | 是否显示清除按钮，当已选非空且未禁用时生效。
`placeholder` | `Option<String>` | 未选择时显示的占位文案。
`disabled` | `bool` | 是否禁用整个选择器，禁用时阻止交互与回调。
`show_search` | `bool` | 是否启用简单搜索，默认 `false`。
`status` | `Option<ControlStatus>` | 视觉状态类，用于展示校验结果（success/warning/error）。
`size` | `Option<ComponentSize>` | 尺寸：Small/Middle/Large，默认跟随全局 ConfigProvider。
`class` / `style` | `Option<String>` | 自定义类名与行内样式，应用到外层容器。
`dropdown_class` / `dropdown_style` | `Option<String>` | 自定义弹层的类与样式。
`on_change` | `Option<EventHandler<Vec<String>>>` | 选中集合变更回调，单选约定 Vec 长度为 0 或 1。

### 受控 / 非受控 / Form 集成逻辑

内部根据当前环境按以下优先级决定选中集合 `selected_keys: Vec<String>`（只读快照）：

1. **Form 场景**：存在 `FormItemControlContext`：
   - 单选：
     - 从 `ctx.value()` 读取 `serde_json::Value`，通过 `value_to_option_key` 转成 `Option<String>`，再包装成 `Vec<String>`（0 或 1 个元素）。
   - 多选：
     - 从 `ctx.value()` 读取数组，使用 `value_to_option_keys` 将 `Value::Array` 转成 `Vec<String>`。
   - 写回：在选中变化时调用 `ctx.set_value(...)`，单选时写入 `Value::String(key)`，多选时写入 `Value::Array([...])`。

2. **受控 props**：
   - 若未在 Form 中，但传入 `values`：直接使用 `values` 作为选中集合；
   - 否则若传入 `value`：使用 `vec![value]` 作为选中集合；

3. **非受控内部 state**：
   - 上述两者都不存在时，使用内部 `Signal<Vec<String>>` `internal_selected` 维护选中集合；
   - 变更时更新 `internal_selected` 并触发 `on_change`。

> 简化后原则：在 Form 中使用时无需传 `value`/`values`，Select 会自动与当前字段绑定；在 Form 外建议统一使用受控模式（props + on_change）。

### 弹层打开/关闭与 Esc / 点击空白

- 打开：
  - 点击触发区域（输入框样式的外层）会切换 `open_state`；
  - 键盘 Enter/ArrowDown 在未打开时会打开弹层。
- 关闭：
  - 单选：点击选项后自动关闭；
  - Esc：在打开状态下按 Esc 将 `open_state` 置为 `false`；
  - 点击空白：使用 `document` 级别的 `click` 监听实现：

    ```rust
    let internal_click_flag: Signal<bool> = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        let mut open_for_global = open_state;
        let mut internal_flag = internal_click_flag;
        use_effect(move || {
            use wasm_bindgen::{closure::Closure, JsCast};

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let target: web_sys::EventTarget = document.into();
                    let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
                        move |_evt: web_sys::MouseEvent| {
                            let mut flag = internal_flag;
                            if *flag.read() {
                                flag.set(false);
                                return;
                            }
                            let mut open_signal = open_for_global;
                            if *open_signal.read() {
                                open_signal.set(false);
                            }
                        },
                    ));
                    let _ = target.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref());
                    handler.forget();
                }
            }
        });
    }
    ```

- 内部交互（点击触发区、选择项、键盘操作）在处理前会将 `internal_click_flag` 设置为 `true`，防止本轮事件被 document 监听误判为空白点击：

  ```rust
  // 触发区点击
  onclick: move |_| {
      if is_disabled_flag { return; }
      let mut flag = internal_click_for_toggle;
      flag.set(true);
      // 切换 open_state
  }

  // 选项点击
  onclick: move |_| {
      if disabled_opt { return; }
      let mut flag = internal_click_for_item;
      flag.set(true);
      // 更新选中集合...
  }
  ```

### 多选模式与清除按钮

- 多选模式下：
  - `multiple = true`；
  - `values: Option<Vec<String>>` 或 Form 中字段值为 `Value::Array([...])`；
  - 选中的项在触发区域以“标签”形式展示（`adui-select-selection-item`），内部渲染为多个 span。
- `allow_clear = true` 时：
  - 当选中集合非空且未禁用时，右侧会出现一个清除按钮（`×`）；
  - 点击清除按钮时会将选中集合置空：

    ```rust
    if allow_clear && !selected_keys.is_empty() && !is_disabled_flag {
        span {
            class: "adui-select-clear",
            onclick: move |_| {
                apply_selected_keys(
                    &form_for_handlers,
                    multiple_flag,
                    controlled_flag,
                    &internal_selected_for_handlers,
                    on_change_cb,
                    Vec::new(),
                );
            },
            "×"
        }
    }
    ```

## 与 Form 集成示例

摘自 `examples/select_demo.rs`：

```rust
#[component]
fn FormSelectSection() -> Element {
    let form_signal = use_signal(use_form);
    let submit_message = use_signal(|| "尚未提交".to_string());

    let options = vec![
        SelectOption { key: "weekly".into(), label: "Weekly".into(), disabled: false },
        SelectOption { key: "monthly".into(), label: "Monthly".into(), disabled: false },
        SelectOption { key: "yearly".into(), label: "Yearly".into(), disabled: false },
    ];

    rsx! {
        div {
            Form {
                layout: FormLayout::Vertical,
                form: Some(form_signal.read().clone()),
                on_finish: {
                    let mut submit_message = submit_message.clone();
                    move |evt: FormFinishEvent| {
                        submit_message.set(format!("提交成功: {:?}", evt.values));
                    }
                },
                FormItem {
                    name: Some("plan".to_string()),
                    label: Some("Plan".to_string()),
                    rules: Some(vec![
                        FormRule {
                            required: true,
                            message: Some("Please select plan".into()),
                            ..FormRule::default()
                        }
                    ]),
                    Select {
                        options: options.clone(),
                        placeholder: Some("Select plan".to_string()),
                    }
                }
                FormItem {
                    name: None,
                    label: None,
                    children: rsx! {
                        Button {
                            r#type: ButtonType::Primary,
                            html_type: ButtonHtmlType::Submit,
                            "Submit"
                        }
                    }
                }
            }

            div {
                strong { "提交结果：" }
                pre {
                    style: "background:#f5f5f5;border:1px solid #ddd;padding:8px;",
                    "{submit_message.read()}"
                }
            }
        }
    }
}
```

要点：

- 未传 `value`/`values`，Select 自动与 `plan` 字段绑定；
- 提交时，`evt.values` 中 `"plan"` 对应的 `Value::String` 即为选中项的 `key`；
- 未选择时提交会触发校验失败，展示错误信息，并给 FormItem 添加错误样式。

## 与 Ant Design 的差异与注意事项

- 当前 `Select` 仅覆盖常用子集：
  - 不支持 `labelInValue`、`mode="tags"` 的复杂语义；
  - 不支持虚拟滚动、大规模远程数据源；
  - 不支持分组（`OptGroup`）与自定义选项渲染（后续可通过 slot/children 方式扩展）。
- 搜索行为为简单前端过滤：
  - 不支持 AntD 中 `filterOption` 的完全自定义过滤函数；
  - 若需自定义搜索，可在外部维护 `options`，通过 `on_change` / 其他事件更新。
- 在 Form 场景下：
  - 推荐始终使用 `key: String` 作为选中标识，避免直接存 `label`；
  - 若需要更复杂的值结构，可以在提交时从 `key` 派生或在业务层做二次映射。

> 后续计划：TreeSelect / Cascader / AutoComplete 会基于同一套 `SelectOption` / `OptionNode` 模型与 Overlay 管理方案，实现树形、多级与输入联想选择器，并在 `docs/select_family.md` 中统一整理设计与 API。