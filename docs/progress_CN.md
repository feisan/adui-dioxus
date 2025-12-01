# Progress 进度条

## 概述

Progress 组件显示操作的进度。它支持线和圆形两种类型，具有不同的状态和可自定义的样式。

## API 参考

### ProgressProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `percent` | `f32` | `0.0` | 百分比，范围 [0.0, 100.0] |
| `status` | `Option<ProgressStatus>` | `None` | 可选状态（当 percent >= 100 时自动为成功） |
| `show_info` | `bool` | `true` | 是否显示文本百分比 |
| `type` | `ProgressType` | `ProgressType::Line` | 进度指示器的视觉类型 |
| `stroke_width` | `Option<f32>` | `None` | 可选的描边宽度（线型为高度，圆形为边框宽度） |
| `class` | `Option<String>` | `None` | 根元素的额外 CSS 类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |

### ProgressStatus

- `Normal` - 正常状态
- `Success` - 成功状态（绿色）
- `Exception` - 异常/错误状态（红色）
- `Active` - 活动/动画状态

### ProgressType

- `Line` - 线型进度条（默认）
- `Circle` - 圆形进度指示器

## 使用示例

### 基础线型进度

```rust
use adui_dioxus::Progress;

rsx! {
    Progress {
        percent: 50.0,
    }
}
```

### 成功状态

```rust
use adui_dioxus::{Progress, ProgressStatus};

rsx! {
    Progress {
        percent: 100.0,
        status: Some(ProgressStatus::Success),
    }
}
```

### 圆形进度

```rust
use adui_dioxus::{Progress, ProgressType};

rsx! {
    Progress {
        percent: 75.0,
        r#type: ProgressType::Circle,
    }
}
```

### 不显示信息文本

```rust
use adui_dioxus::Progress;

rsx! {
    Progress {
        percent: 60.0,
        show_info: false,
    }
}
```

### 异常状态

```rust
use adui_dioxus::{Progress, ProgressStatus};

rsx! {
    Progress {
        percent: 50.0,
        status: Some(ProgressStatus::Exception),
    }
}
```

## 使用场景

- **文件上传**：显示上传进度
- **表单提交**：显示提交进度
- **数据加载**：显示数据加载进度
- **任务完成**：指示任务完成百分比

## 与 Ant Design 6.0.0 的差异

- ✅ 线和圆形类型
- ✅ 状态变体
- ✅ 自定义描边宽度
- ✅ 百分比显示
- ⚠️ 仪表盘类型尚未实现
- ⚠️ 某些高级样式选项可能有所不同

