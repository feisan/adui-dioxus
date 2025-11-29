# Rate

评分组件，支持半星与可清空，受控/非受控与表单集成。

## Props 要点
- `value` / `default_value`：`Option<f64>`。
- `count`（默认 5）、`allow_half`、`allow_clear`、`disabled`。
- `character`：自定义字符/节点；`tooltips`：每项的提示文本。
- 事件：`on_change`、`on_hover_change`，聚焦/失焦回调。

## 交互与可访问性
- 指针：支持半星（基于左半/右半区域），再次点击可清空（allow_clear）。
- 键盘：Left/Down 减一步，Right/Up 加一步，Home 到 0，End 到最大，Enter/Space 提交当前值。
- 禁用：禁用态不响应交互，样式变灰。

## 示例
- 见 `examples/rate/main.rs`：半星受控、禁用状态。 
