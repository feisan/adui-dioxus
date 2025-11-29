# InputNumber

数值输入框，支持步进、边界与精度约束，受控/非受控与表单集成。

## Props 要点
- `value` / `default_value`：受控/非受控值，`Option<f64>`。
- `min` / `max` / `step` / `precision`：边界、步进与小数精度控制。
- `controls`：是否显示上下步进按钮（默认开启）。
- `prefix` / `suffix`：前后缀节点。
- `disabled` / `status`：禁用与状态样式。
- 事件：`on_change`（`Option<f64>`）、`on_change_complete`（提交时），支持键盘上下箭头/Enter。

## 交互与可访问性
- 键盘：↑/↓ 步进，Enter 提交，受控/表单值变化会同步输入框显示。
- 边界：超出 min/max 自动 clamp；精度控制会在显示时格式化。
- 禁用：禁用态禁用输入和按钮，样式变灰。

## 示例
- 见 `examples/input_number/main.rs`：基础、禁用、受控同步显示。 
