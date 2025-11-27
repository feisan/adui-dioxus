# App & ConfigProvider & 全局反馈（设计草案）

> 本文是 Phase 3（`plan/0006.md`） 的设计说明，用于指导 `App` / `ConfigProvider` / `Message` / `Notification` / `Modal` / `Drawer` 的迁移与实现。后续实现落地后可根据实际情况调整。

## 1. 角色划分

- `ConfigProvider`
  - 职责：
    - 提供全局 UI 配置（尺寸、禁用状态、样式前缀、Locale、主题 Token 等）；
    - 为各组件提供一致的 `get_prefix_cls` 与主题能力；
    - 类似 Ant Design 的 `ConfigProvider`，但会与现有 `ThemeProvider` 协同。
  - 使用方式：
    - 作为应用根节点或某个子树的包裹组件；
    - 组件内部通过 `use_config()` 读取当前配置。

- `App`
  - 职责：
    - 挂载全局 overlay 容器（消息、通知、模态框等）；
    - 提供 `use_message` / `use_notification` / `use_modal` 等 hook 风格的全局操作接口；
    - 为静态方法风格的 API 提供上下文（替代直接使用全局单例）。
  - 使用方式：
    - 一般包裹在 `ConfigProvider` 内部或与之组合使用；
    - 子组件中调用 `use_message()` 等获取 API。

- `Message` / `Notification`
  - 职责：
    - `Message`：轻量级全局提示（操作结果、表单提交反馈等）；
    - `Notification`：较重的全局通知（带标题、描述、操作区域的右上角通知）。
  - 使用方式：
    - 通过 `use_message()` / `use_notification()` 获取 API；
    - 或通过 `App` 暴露的静态/半静态方法（后续扩展）。

- `Modal` / `Drawer`
  - 职责：
    - `Modal`：页面级模态对话框，常与表单结合使用；
    - `Drawer`：从侧边滑出的抽屉式容器，用于较复杂的编辑界面或导航。
  - 使用方式：
    - 受控组件形式：`open` + `on_ok` / `on_cancel`；
    - 未来可能扩展 `Modal.confirm` 等静态方法形式（依赖 `App`）。

## 2. MVP 范围

### 2.1 ConfigProvider（MVP）

- 支持的核心配置：
  - `size`: 控制默认尺寸（`small` / `middle` / `large`），影响 Button / Input / Modal 等；
  - `disabled`: 全局禁用标记，控制交互类组件的默认禁用态；
  - `prefix_cls`: 样式前缀（可简化为字符串，不支持复杂动态逻辑）；
  - `locale`: 预留结构，先只覆盖少量常用文案（如空状态、表单校验提示的语言切换）；
  - 与现有 `ThemeProvider` 对齐的主题 token 映射（颜色、圆角、间距等）。
- 暂不支持：
  - 完整的动态主题算法链（algorithm 数组等）；
  - 全量组件级别的 theme 配置（按组件细粒度 token）；
  - 与 css-in-js 引擎深度集成的运行时主题切换。

### 2.2 App（MVP）

- 支持：
  - 作为全局容器，内部挂载 overlay root；
  - 在 context 中提供 `message` / `notification` / `modal` API 实例；
  - 提供 `use_app()` / `use_message()` / `use_notification()` / `use_modal()` 等 hook。
- 暂不支持：
  - 多重 App 嵌套下的复杂 overlay 继承逻辑（先保证简单父子场景可用）；
  - 与动态主题/多语言在运行时切换时的过渡动画处理。

### 2.3 Message / Notification（MVP）

- Message：
  - API：`open`, `success`, `info`, `warning`, `error`, `loading`, `destroy`；
  - 支持：
    - 文本内容 + 可选图标；
    - `duration` 自动关闭；
    - `on_close` 回调；
    - 简单队列管理与最大数量（`max_count`）。
- Notification：
  - API：`open`, `success`, `info`, `warning`, `error`, `close`, `destroy`；
  - 支持：
    - 标题 + 描述 + 图标；
    - 基本的 `placement`（先实现 `topRight` / `bottomRight`）；
    - 自动/手动关闭；
    - `on_close` 回调。
- 暂不支持：
  - Promise 风格的链式调用（`then` / `update` 复杂玩法）；
  - 进度条、暂停悬停等高级功能（Notification 的 `showProgress` / `pauseOnHover` 等）；
  - 基于 `holderRender` 的深度自定义挂载点（先提供简单容器配置）。

### 2.4 Modal / Drawer（MVP）

- Modal：
  - 受控 props：`open`, `on_ok`, `on_cancel`, `title`, `footer`, `closable`, `mask_closable`, `destroy_on_close`；
  - 支持：
    - 基本动画与 mask；
    - ESC 关闭；
    - 与 Form 组合使用的典型场景（表单提交成功后自动关闭与触发 message）。
- Drawer：
  - 受控 props：`open`, `on_close`, `title`, `placement`, `width` / `height`, `destroy_on_close` 等；
  - 支持：
    - 左/右/上/下几种常见 `placement`；
    - 简单的尺寸配置和 mask 行为；
    - 与 Modal 共用 z-index / overlay 管理策略。
- 暂不支持：
  - 可拖拽、可缩放 Drawer；
  - Modal.confirm / info / success / error / warning 等静态方法风格 API（先做受控组件，后续依赖 App 能力迭代）。

## 3. OverlayManager 抽象

`OverlayManager` 是一个轻量的内部抽象，用来在单个 App 树内统一管理浮层的 z-index 和基本元信息：

- 只关心浮层的种类（`Message` / `Notification` / `Modal` / `Drawer`）和元数据（`z_index`、是否有 `mask`）。
- 不直接持有具体 UI，而是为上层组件（Message/Notification/Modal/Drawer）提供：
  - `open(kind, has_mask) -> (key, meta)`：登记一个新的浮层并返回唯一 key 与分配的 z-index；
  - `update(key, has_mask?)`：更新已登记浮层的部分元信息；
  - `close(key)` / `close_all()`：关闭单个或全部浮层；
  - `entries()` / `current_top_z_index()`：用于渲染层和调试。
- 通过 `OverlayHandle` + Dioxus 的 `Signal` 集成到组件树中：
  - 顶层使用 `use_overlay_provider()` 创建并注入 context；
  - 子组件通过 `use_overlay()` 取得 `OverlayHandle`，再调用 `open/close` 等方法。

在后续实现中，Message/Notification/Modal/Drawer 不必关心各自 z-index 的互相竞争问题，只需要向 `OverlayManager` 申请和释放层级即可。

## 4. 与现有能力的关系

### 4.1 App 与 use_* API

- 使用方式：
  - 在应用根部包裹：`ConfigProvider { App { .. } }`（或先 `App` 再内部自己挂 ConfigProvider）；
  - 在任意子组件中：
    - `let msg = use_message();`
    - `let notice = use_notification();`
    - `let modal = use_modal();`。
- 设计要点：
  - `App` 内部通过 `use_overlay_provider()` 安装一个 OverlayManager，确保所有全局反馈组件共享同一套 z-index 管理；
  - `use_app()` 返回完整的 `AppContextValue`，`use_message` / `use_notification` / `use_modal` 只是便捷封装；
  - 当前阶段 Message/Notification/Modal 的 API 是 MVP 占位实现（例如 `open()` 仅登记 overlay 层级），视觉与完整行为在后续 E/F 小节实现。
- 示例：
  - `examples/app_demo.rs` 展示了在 App 子树中调用 `use_message` / `use_notification` / `use_modal` 的基本方式。

### 4.2 ConfigProvider 与 ThemeProvider

- `ConfigProvider` 当前支持：
  - `size`: small / middle / large（映射到 `ComponentSize`），用于控制 Button 等组件的默认尺寸；
  - `disabled`: 全局禁用标记，Input 等组件会在自身 `disabled` 为 false 时仍遵守全局禁用；
  - `prefix_cls`: 目前仅在样式命名上预留，对现有 `adui-` 前缀保持兼容。
- 优先级规则：
  - 组件本地 props（如 `ButtonProps.size`、`InputProps.disabled`）优先于全局配置；
  - `ButtonGroup` 中的 size/shape 优先级高于 ConfigProvider 的全局 size；
  - Form 内部的 `disabled`（如 FormItem 控制字段）与 ConfigProvider 的 `disabled` 组合生效，任一为 true 即禁用。

- 与 `ThemeProvider`：
  - ConfigProvider 会优先复用现有 ThemeProvider 中的 token 和主题定义，避免重复配置；
  - ThemeProvider 仍然是底层“主题来源”，ConfigProvider 侧重于 UI 语义配置（size、disabled、prefix、locale 等）。

- 与 Form / Input 等：
  - Modal / Drawer 的设计会优先满足「弹窗表单」场景，保持与当前 Form 验证模型兼容；
  - Message / Notification 将作为 Form 提交结果的主要反馈方式之一，在文档示例中重点展示。

## 5. 全局反馈：Message / Notification

- 当前 Message 能力：
  - API：`open(MessageConfig)` + `info/success/warning/error/loading` 便捷方法；
  - 自动关闭：除 `loading` 外，默认按 `duration`（秒）通过 `setTimeout` 定时关闭（仅 Web 目标）；
  - 手动关闭：`destroy(Some(key))` 关闭单条，`destroy(None)` 关闭所有；
  - 渲染：基于 `OverlayManager` 的 z-index，在右上角堆叠展示简单文本消息。
- 当前 Notification 能力：
  - API：`open(NotificationConfig)` + `info/success/warning/error`；
  - 自动关闭：按 `duration` 定时关闭（仅 Web 目标）；
  - 位置：支持 `topRight` 与 `bottomRight` 两个 placement；
  - 渲染：标题 + 描述 + 关闭按钮，样式为 MVP 版本。
- 与 AntD 的主要差异：
  - 暂不支持 Promise 风格链式调用与 `update`；
  - 全局配置能力仅覆盖少量常用字段（通过 App/ConfigProvider 间接配置）；
  - 样式与动效为精简版本，后续可参考 AntD 的 CSSMotion 行为迭代。

## 6. 浮层组件：Modal / Drawer

- Modal：
  - 受控使用：`ModalProps { open, title, footer, on_ok, on_cancel, closable, mask_closable, destroy_on_close, .. }`；
  - 行为：
    - `open=true` 时渲染 mask + 内容层，使用 OverlayManager 分配 z-index；
    - mask 点击（在 `mask_closable=true` 时）和 ESC 按键触发 `on_cancel`；
    - `destroy_on_close=true` 时关闭后卸载内容；
  - 当前只提供最小 header/body/footer 布局，未内置默认 OK/Cancel 按钮。
- Drawer：
  - 受控使用：`DrawerProps { open, title, on_close, placement, size, mask_closable, destroy_on_close, .. }`；
  - 支持 `placement: Left/Right/Top/Bottom`，通过 `size` 控制宽/高；
  - 行为：
    - `open=true` 时渲染 mask + 侧边面板，同样通过 OverlayManager 分配 z-index；
    - mask 点击（在 `mask_closable=true` 时）触发 `on_close`；
  - 当前无动画，仅通过固定定位实现基础布局。
- 示例：
  - `examples/modal_demo.rs`：基础受控 Modal；
  - `examples/drawer_demo.rs`：右侧 Drawer 示例。

## 7. 后续可扩展方向（非本轮目标）

- 完整的动态主题算法与组件级 token 配置；
- 更丰富的 Notification 能力（进度、悬停暂停、分组等）；
- 完整对齐 AntD 的 `Modal.confirm` 系列静态方法与 Promise 接口；
- 更灵活的挂载点配置与多容器 overlay 支持；
- 与路由、全局状态管理框架的更深层集成（如自动在路由切换时销毁 overlay）。
