# 选择器家族概览（Select / TreeSelect / Cascader / AutoComplete）

本节作为「选择器家族」的汇总文档，补充 `Select`、`TreeSelect`、`Cascader`、`AutoComplete` 等组件之间的关系与设计取舍。详细 API 请分别参考：

- `Select`：`docs/select.md`（已完成）；
- `TreeSelect`：本节内「TreeSelect：树形选择」小节；
- `Cascader`：本节内「Cascader：级联选择」小节；
- `AutoComplete`：本节内「AutoComplete：输入联想」小节。

## 设计目标回顾

结合 `Form` / `OverlayManager` / `ConfigProvider`，选择器家族共享一套基础能力：

- 统一的选项与值模型：
  - 使用 `OptionKey = String` 作为统一 key；
  - 扁平列表使用 `SelectOption { key, label, disabled }`；
  - 树/级联使用 `TreeNode`/`CascaderNode`（带 children 的树节点）。
- 统一的下拉基础设施：
  - 通过 `OverlayManager` 管理 z-index 与挂载容器；
  - 支持 Esc 关闭、点击空白关闭、键盘上下选择；
  - 复用统一的 CSS 样式（`.adui-select-*`）。
- 与 Form 的一致集成：
  - 值统一映射到 `serde_json::Value`，与现有 `Form`、`Checkbox`、`Radio` 对齐；
  - Form 场景下由 `FormItemControlContext` 统一读写字段值，组件不额外维护受控 state。

> 一句话：所有选择器都在统一「Select 基础设施」上构建，只是在选项结构（扁平/树/路径）与交互形态上有所不同。

---

## TreeSelect：树形选择（MVP）

> 实现位置：`src/components/tree_select.rs`
>
> 示例：`examples/tree_select_demo.rs`

（保留原有 TreeSelect 小节内容，这里省略）

---

## Cascader：级联选择（MVP）

> 实现位置：`src/components/cascader.rs`
>
> 示例：`examples/cascader_demo.rs`

（保留上一轮已经写好的 Cascader 小节内容，这里省略）

---

## AutoComplete：输入联想（MVP）

> 实现位置：`src/components/auto_complete.rs`
>
> 示例：`examples/auto_complete_demo.rs`

### 能力范围

当前 AutoComplete 为基于 Input 的联想输入，重点覆盖：

- 受控 / 非受控输入：
  - `value: Option<String>` 受控模式；
  - `default_value: Option<String>` 非受控初始值；
  - Form 场景下优先从当前字段的 `Value` 中读取字符串（通过 `form_value_to_string`）。
- 本地候选过滤：
  - 通过 `options: Option<Vec<SelectOption>>` 提供候选项；
  - 默认根据当前输入值对 options 的 label 做包含匹配（忽略大小写），复用 `filter_options_by_query`。
- 选择行为：
  - 点击候选项时将其 `label` 写回输入框；
  - 触发 `on_select(key)` 与 `on_change(text)`，并关闭下拉；
  - Form 场景下同步将 `text` 写回 FormStore。
- 关闭行为：
  - 按 Esc 关闭下拉；
  - 点击空白关闭（document 级别 click + `internal_click_flag`）。

### AutoCompleteProps 核心字段

> 类型定义：`src/components/auto_complete.rs:17` 起。

字段 | 类型 | 说明
--- | --- | ---
`options` | `Option<Vec<SelectOption>>` | 候选列表，使用 SelectOption 统一模型；`None` 时不展示下拉。
`value` | `Option<String>` | 受控输入值。（Form 场景通常不需要显式传入）
`default_value` | `Option<String>` | 非受控初始值，仅在未受控且非 Form 场景下生效。
`placeholder` | `Option<String>` | 占位文本。
`allow_clear` | `bool` | 是否展示清除按钮，输入不为空且未禁用时显示。
`disabled` | `bool` | 是否禁用整个组件。
`status` | `Option<ControlStatus>` | 视觉状态（success/warning/error），影响边框样式。
`size` | `Option<ComponentSize>` | 组件尺寸，默认跟随 ConfigProvider。
`class` / `style` | `Option<String>` | 自定义类名与样式，应用到触发区容器。
`dropdown_class` / `dropdown_style` | `Option<String>` | 自定义下拉弹层的类名与样式。
`on_change` | `Option<EventHandler<String>>` | 输入值变化回调，在写回 Form 或内部 state 之后触发。
`on_search` | `Option<EventHandler<String>>` | 输入变化时触发，可用于外部异步更新 `options`。
`on_select` | `Option<EventHandler<String>>` | 选择候选项时触发，参数为该项 `key`。

### 值模型与 Form 映射

AutoComplete 将自身视作「字符串输入框」，与 Form 集成方式与 `Input` 一致：

- 读取：
  - 若存在 `FormItemControlContext`，通过 `form_value_to_string(ctx.value())` 获取当前字段值；
  - 否则若传入 `value`，使用受控值；
  - 否则使用内部 `Signal<String>` 作为非受控值。
- 写回：

  ```rust
  let handle_input_change = move |next: String| {
      if let Some(ctx) = form_for_handlers.as_ref() {
          ctx.set_string(next.clone());
      } else if !controlled_by_prop {
          let mut inner = inner_for_change;
          inner.set(next.clone());
      }
      if let Some(cb) = on_change_cb {
          cb.call(next.clone());
      }
      if let Some(cb) = on_search_cb {
          cb.call(next);
      }
  };
  ```

> 在 Form 场景下，建议只指定 `FormItem.name`，不再额外传入 `value`，由 FormStore 管理字符串值；在 Form 外使用时，推荐统一采用受控模式（`value` + `on_change`）。

### 触发区和下拉行为

- 触发区：
  - 使用 `.adui-select-root` + `.adui-select` 包裹 Input，使视觉风格与 Select 对齐；
  - 内部实际渲染的是一个 `<input class="adui-input">`，可响应 `onfocus`/`oninput`/`onkeydown`。
- 下拉展示：
  - 仅当 `open_state = true` 且 `filtered_options` 非空时渲染；
  - 每个候选项使用 `.adui-select-item`，若禁用则加 `.adui-select-item-option-disabled`；
  - 点击候选项：
    - 将 `label` 写入输入框与 Form/内部 state；
    - 调用 `on_change(label)` 与 `on_select(key)`；
    - 并关闭下拉。

### 示例：基础用法与 Form 集成

摘自 `examples/auto_complete_demo.rs`：

```rust
fn city_options() -> Vec<SelectOption> {
    vec![
        SelectOption { key: "hangzhou".into(), label: "Hangzhou".into(), disabled: false },
        SelectOption { key: "shanghai".into(), label: "Shanghai".into(), disabled: false },
        SelectOption { key: "beijing".into(), label: "Beijing".into(), disabled: false },
        SelectOption { key: "shenzhen".into(), label: "Shenzhen".into(), disabled: false },
    ]
}

#[component]
fn BasicAutoCompleteSection() -> Element {
    let options = city_options();

    let mut value = use_signal(|| "Hangzhou".to_string());
    let mut last_search = use_signal(|| String::new());
    let mut last_select = use_signal(|| String::new());

    AutoComplete {
        options: Some(options),
        value: Some(value.read().clone()),
        placeholder: Some("请输入城市".to_string()),
        on_change: move |txt: String| {
            value.set(txt);
        },
        on_search: move |txt: String| {
            last_search.set(txt);
        },
        on_select: move |key: String| {
            last_select.set(key);
        },
    }
}

#[component]
fn FormAutoCompleteSection() -> Element {
    let form_signal = use_signal(use_form);

    Form {
        layout: FormLayout::Vertical,
        form: Some(form_signal.read().clone()),
        FormItem {
            name: Some("city".to_string()),
            label: Some("City".to_string()),
            rules: Some(vec![
                FormRule {
                    required: true,
                    message: Some("请输入或选择城市".into()),
                    ..FormRule::default()
                }
            ]),
            AutoComplete {
                options: Some(city_options()),
                placeholder: Some("请输入城市".to_string()),
            }
        }
    }
}
```

### 与 Ant Design 的差异与后续扩展点

当前 AutoComplete 与 Ant Design 6.x 相比，仍是一个简化版本：

- 未实现：
  - 自定义 `value` / `option` 结构（目前仅支持基于 `SelectOption` 的简单 key/label 模型）；
  - 复杂过滤策略、远程搜索的 loading/空态/防抖控制；
  - 完整键盘导航（当前仅支持 Esc 关闭，不支持上下键在候选项中移动与 Enter 选中）；
  - 自定义渲染（如 `option` 的自定义模板）。
- 已对齐或刻意保持一致的行为：
  - Form 集成方式与 `Input` 对齐，使用字符串值；
  - 关闭策略与 Select/TreeSelect/Cascader 一致（Esc/点击空白）；
  - 视觉样式沿用 Select/Input 的 token 与类名。

后续如需要更复杂的远程联想或大数据场景，可在现有实现基础上扩展 `on_search` 协议、增加 loading/空态与键盘导航能力。