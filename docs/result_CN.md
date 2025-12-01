# Result 结果

## 概述

Result 组件显示操作的结果，如成功、错误或其他状态。通常用于空状态、错误页面和成功确认。

## API 参考

### ResultProps

| 属性        | 类型                   | 默认值                  | 说明                            |
| ----------- | ---------------------- | ----------------------- | ------------------------------- |
| `status`    | `Option<ResultStatus>` | `None`（默认为 `Info`） | 结果的整体状态                  |
| `icon`      | `Option<Element>`      | `None`                  | 可选的自定义图标                |
| `title`     | `Option<Element>`      | `None`                  | 结果页面的标题                  |
| `sub_title` | `Option<Element>`      | `None`                  | 可选的副标题/描述文本           |
| `extra`     | `Option<Element>`      | `None`                  | 额外的操作区域，通常是按钮      |
| `class`     | `Option<String>`       | `None`                  | 根元素的额外类名                |
| `style`     | `Option<String>`       | `None`                  | 根元素的内联样式                |
| `children`  | `Option<Element>`      | `None`                  | 在 extra 下方渲染的可选内容区域 |

### ResultStatus

- `Success` - 成功状态
- `Info` - 信息状态（默认）
- `Warning` - 警告状态
- `Error` - 错误状态
- `NotFound` - 404 未找到
- `Forbidden` - 403 禁止访问
- `ServerError` - 500 服务器错误

## 使用示例

### 成功结果

```rust
use adui_dioxus::{Result, ResultStatus, Button, ButtonType};

rsx! {
    Result {
        status: Some(ResultStatus::Success),
        title: Some(rsx!("操作成功")),
        sub_title: Some(rsx!("您的操作已成功完成。")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "返回首页"
            }
        }),
    }
}
```

### 错误结果

```rust
use adui_dioxus::{Result, ResultStatus};

rsx! {
    Result {
        status: Some(ResultStatus::Error),
        title: Some(rsx!("提交失败")),
        sub_title: Some(rsx!("请检查并修改以下信息后重新提交。")),
    }
}
```

### 404 未找到

```rust
use adui_dioxus::{Result, ResultStatus, Button, ButtonType};

rsx! {
    Result {
        status: Some(ResultStatus::NotFound),
        title: Some(rsx!("404")),
        sub_title: Some(rsx!("抱歉，您访问的页面不存在。")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "返回首页"
            }
        }),
    }
}
```

## 使用场景

- **成功页面**：确认成功操作
- **错误页面**：显示错误状态（404、403、500）
- **空状态**：显示无数据可用时
- **操作结果**：显示操作结果

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础结果状态
- ✅ 自定义图标
- ✅ 标题和副标题
- ✅ 额外操作区域
- ⚠️ 某些高级样式选项可能有所不同

