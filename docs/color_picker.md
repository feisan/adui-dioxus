# ColorPicker

单色取色器，支持饱和度/明度面板、色相与透明度滑条，Hex 输入与清空。

## Props 要点
- `value` / `default_value`：十六进制字符串（`#RRGGBB` 或 `#RRGGBBAA`），`Option<String>`。
- `allow_clear`：允许清空（默认开启）。
- `disabled`：禁用交互。
- 事件：`on_change`（实时），`on_change_complete`（提交/失焦）。

## 交互与可访问性
- 面板：拖动饱和度/明度区域更新 `S/V`，色相滑条更新 `H`，透明度滑条更新 `A`。
- 输入：Hex 文本实时解析，非法输入被忽略；清空按钮写入空值。
- 受控/表单：外部/表单值变化时同步预览与输入框。
- 非 wasm 环境：退化为输入驱动。

## 示例
- 见 `examples/color_picker/main.rs`：基础受控与禁用示例。 
