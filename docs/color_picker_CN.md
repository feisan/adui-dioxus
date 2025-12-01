# ColorPicker 颜色选择器

## 概述

ColorPicker 组件提供颜色选择界面，支持 HSVA（色相、饱和度、明度、透明度）颜色模型。它包括调色板、饱和度/明度选择器、色相滑块和透明度滑块。

## API 参考

### ColorPickerProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 受控颜色字符串（例如 #RRGGBB 或 #RRGGBBAA） |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始值 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `allow_clear` | `bool` | `false` | 显示清除按钮 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 每次改变时调用，传入十六进制字符串 |
| `on_change_complete` | `Option<EventHandler<String>>` | `None` | 交互完成时调用 |

## 使用示例

### 基础颜色选择器

```rust
use adui_dioxus::ColorPicker;
use dioxus::prelude::*;

let color = use_signal(|| None::<String>);

rsx! {
    ColorPicker {
        value: color.read().clone(),
        on_change: Some(move |c| {
            color.set(Some(c));
        }),
    }
}
```

### 带清除按钮

```rust
use adui_dioxus::ColorPicker;

rsx! {
    ColorPicker {
        allow_clear: true,
        default_value: Some("#FF0000".to_string()),
    }
}
```

### 带完成处理器

```rust
use adui_dioxus::ColorPicker;

rsx! {
    ColorPicker {
        on_change_complete: Some(move |color| {
            println!("颜色选择完成: {}", color);
        }),
    }
}
```

## 使用场景

- **主题定制**：定制主题颜色
- **设计工具**：设计工具中的颜色选择
- **表单输入**：表单中的颜色输入
- **设置**：应用程序中的颜色设置

## 与 Ant Design 6.0.0 的差异

- ✅ HSVA 颜色模型
- ✅ 饱和度/明度选择器
- ✅ 色相滑块
- ✅ 透明度滑块
- ✅ 十六进制颜色格式
- ⚠️ 某些高级功能可能有所不同

