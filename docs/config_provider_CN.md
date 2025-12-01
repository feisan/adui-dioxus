# ConfigProvider 全局化配置

## 概述

ConfigProvider 组件为其子树中的所有组件提供全局配置。它包装 ThemeProvider，允许设置尺寸、禁用状态、前缀类和语言环境。

## API 参考

### ConfigProviderProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `size` | `Option<ComponentSize>` | `None` | 组件的全局默认尺寸 |
| `disabled` | `Option<bool>` | `None` | 全局禁用标志（为 true 时，交互组件将被禁用，除非显式覆盖） |
| `prefix_cls` | `Option<String>` | `None` | 全局 CSS 类名前缀（默认为 "adui"） |
| `locale` | `Option<Locale>` | `None` | 基本 UI 语言的全局语言环境标志 |
| `theme` | `Option<Theme>` | `None` | 可选的初始主题 |
| `children` | `Element` | - | 子组件（必需） |

### ComponentSize

- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

### Locale

- `ZhCN` - 简体中文（默认）
- `EnUS` - 英语（美国）

## 使用示例

### 基础 ConfigProvider

```rust
use adui_dioxus::{ConfigProvider, ComponentSize};

rsx! {
    ConfigProvider {
        size: Some(ComponentSize::Large),
        Button {
            "大按钮"
        }
    }
}
```

### 全局禁用

```rust
use adui_dioxus::ConfigProvider;

rsx! {
    ConfigProvider {
        disabled: Some(true),
        Button {
            "禁用按钮"
        }
    }
}
```

### 自定义前缀

```rust
use adui_dioxus::ConfigProvider;

rsx! {
    ConfigProvider {
        prefix_cls: Some("custom".to_string()),
        Button {
            "自定义前缀"
        }
    }
}
```

### 带主题

```rust
use adui_dioxus::{ConfigProvider, Theme};

rsx! {
    ConfigProvider {
        theme: Some(Theme::Dark),
        div {
            "深色主题内容"
        }
    }
}
```

### 嵌套 ConfigProvider

```rust
use adui_dioxus::{ConfigProvider, ComponentSize};

rsx! {
    ConfigProvider {
        size: Some(ComponentSize::Middle),
        div {
            "中等尺寸"
            ConfigProvider {
                size: Some(ComponentSize::Small),
                Button {
                    "小按钮"
                }
            }
        }
    }
}
```

## 使用场景

- **全局配置**：为所有组件设置全局默认值
- **主题管理**：提供主题上下文
- **尺寸控制**：全局控制组件尺寸
- **本地化**：为组件设置语言环境

## 与 Ant Design 6.0.0 的差异

- ✅ 全局尺寸配置
- ✅ 全局禁用状态
- ✅ 自定义前缀类
- ✅ 语言环境支持
- ✅ 主题集成
- ⚠️ 相比 Ant Design 的完整 ConfigProvider 已简化
- ⚠️ 某些高级功能可能有所不同

