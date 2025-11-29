# Slider

滑块组件，支持单滑块与范围模式，键盘/拖拽/轨道点击。

## Props 要点
- `value` / `default_value`：`SliderValue::Single(f64)` 或 `Range(f64, f64)`。
- `range`：开启范围模式渲染两个手柄。
- `min` / `max` / `step` / `precision` / `reverse` / `vertical`。
- `marks: Vec<SliderMark>`：刻度与可选 `dots`。
- `disabled`：禁用交互。
- 事件：`on_change`（拖拽/点击/键盘）与 `on_change_complete`（释放/点击）。

## 交互与可访问性
- 键盘：Left/Right/Up/Down 步进，Home/End 跳转；支持 range 时的单手柄聚焦切换。
- 指针：轨道点击、拖拽；范围模式会自动选择最近手柄。
- ARIA：手柄 role=slider，设置了 `aria-valuemin/max/now`。

## 示例
- 见 `examples/slider/main.rs`：单滑块、范围、禁用、竖向与 marks+dots。 
