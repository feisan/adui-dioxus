# Progress

## Overview

The Progress component displays the progress of an operation. It supports both line and circle types, with different statuses and customizable styling.

## API Reference

### ProgressProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `percent` | `f32` | `0.0` | Percentage in range [0.0, 100.0] |
| `status` | `Option<ProgressStatus>` | `None` | Optional status (auto-success when percent >= 100) |
| `show_info` | `bool` | `true` | Whether to render textual percentage |
| `type` | `ProgressType` | `ProgressType::Line` | Visual type of progress indicator |
| `stroke_width` | `Option<f32>` | `None` | Optional stroke width (height for line, border width for circle) |
| `class` | `Option<String>` | `None` | Extra CSS class name on root |
| `style` | `Option<String>` | `None` | Inline style on root |

### ProgressStatus

- `Normal` - Normal status
- `Success` - Success status (green)
- `Exception` - Exception/error status (red)
- `Active` - Active/animated status

### ProgressType

- `Line` - Line progress bar (default)
- `Circle` - Circular progress indicator

## Usage Examples

### Basic Line Progress

```rust
use adui_dioxus::Progress;

rsx! {
    Progress {
        percent: 50.0,
    }
}
```

### Success Status

```rust
use adui_dioxus::{Progress, ProgressStatus};

rsx! {
    Progress {
        percent: 100.0,
        status: Some(ProgressStatus::Success),
    }
}
```

### Circle Progress

```rust
use adui_dioxus::{Progress, ProgressType};

rsx! {
    Progress {
        percent: 75.0,
        r#type: ProgressType::Circle,
    }
}
```

### Without Info Text

```rust
use adui_dioxus::Progress;

rsx! {
    Progress {
        percent: 60.0,
        show_info: false,
    }
}
```

### Exception Status

```rust
use adui_dioxus::{Progress, ProgressStatus};

rsx! {
    Progress {
        percent: 50.0,
        status: Some(ProgressStatus::Exception),
    }
}
```

## Use Cases

- **File Uploads**: Show upload progress
- **Form Submissions**: Display submission progress
- **Data Loading**: Show data loading progress
- **Task Completion**: Indicate task completion percentage

## Differences from Ant Design 6.0.0

- ✅ Line and circle types
- ✅ Status variants
- ✅ Custom stroke width
- ✅ Percentage display
- ⚠️ Dashboard type not yet implemented
- ⚠️ Some advanced styling options may differ

