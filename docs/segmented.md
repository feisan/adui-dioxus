# Segmented

按钮分段选择，单选模式，支持 block/round 与图标。

## Props 要点
- `options: Vec<SegmentedOption>`：包含 `label`、`value`、可选 `icon`/`tooltip`/`disabled`。
- `value` / `default_value`：受控/非受控；`block` 填充父宽；`round` 圆角；`disabled`。
- 事件：`on_change` 返回选中的 `value`。

## 交互与可访问性
- 键盘：Left/Up/Right/Down 在可用项间循环切换。
- 禁用：整体或单项禁用，忽略点击与键盘。
- 受控：外部 value 变更会同步选中态。

## 示例
- 见 `examples/segmented/main.rs`：基础、Block、禁用。 
