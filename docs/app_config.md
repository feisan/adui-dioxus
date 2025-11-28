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
  - `locale`: 简化版 `Locale` 枚举（`ZhCN` / `EnUS`），当前主要用于日期时间组件的占位文案与星期/月标题语言切换，未来可扩展到更多提示文案；
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

- 只关心浮层的种类（`Message` / `Notification` / `Modal` / `Drawer` / `Dropdown` 等）和元数据（`z_index`、是否有 `mask` 等）；
- 不直接持有具体 UI，而是为上层组件提供：
  - `open(kind, has_mask) -> (key, meta)`：登记一个新的浮层并返回唯一 key 与分配的 z-index；
  - `update(key, has_mask?)`：更新已登记浮层的部分元信息；
  - `close(key)` / `close_all()`：关闭单个或全部浮层；
  - `entries()` / `current_top_z_index()`：用于渲染层和调试。
- 通过 `OverlayHandle` + Dioxus 的 `Signal` 集成到组件树中：
  - 顶层使用 `use_overlay_provider()` 创建并注入 context；
  - 子组件通过 `use_overlay()` 取得 `OverlayHandle`，再调用 `open/close` 等方法。

在后续实现中，Message/Notification/Modal/Drawer/选择器家族组件不必关心各自 z-index 策略，而是统一委托给 OverlayManager —— 每一个“浮层实例”只需要注册自己，剩余交由管理器调度。

### 3.1 下拉类 Overlay（已实现）

- Select / TreeSelect / Cascader / AutoComplete 等组件，通过 `use_dropdown_layer(open: bool)` 与 OverlayManager 集成：
  - 当 `open` 为 true 时，`use_dropdown_layer` 会调用 `overlay.open(OverlayKind::Dropdown, false)` 登记一个下拉浮层，并返回其 `z_index`；
  - 当 `open` 变为 false 时，对应的 overlay entry 会被关闭和清理；
  - 组件只需在渲染下拉面板时使用返回的 `z_index`，而不关心 overlay 的生命周期细节。
- 下拉类组件当前统一采用：
  - `position: absolute` + `top: 100%; left: 0` 的简单定位；
  - click-outside + ESC 关闭机制（见下文浮层体系规划），确保多个下拉不会互相影响。

### 3.2 浮层体系规划（Tooltip / Popover / Popconfirm / Dropdown）

> 本小节对应 `plan/0008.md` 的 Phase 5，主要规划「提示类浮层」与现有 OverlayManager 的集成策略。

- 新增浮层类型：
  - 计划在 `OverlayKind` 中增加用于提示类组件的变种：
    - `Tooltip`：轻量提示气泡，不含交互；
    - `Popup`（或类似命名）：用于 Popover / Popconfirm 等富内容浮层；
    - `DropdownMenu`：用于菜单型 Dropdown；
  - 也可以保留统一的 `OverlayKind::Popup`，再通过 meta 中的 `role` 字段区分具体用途（Tooltip/Popover/Popconfirm/DropdownMenu）。

- 通用浮层 Hook：
  - 在现有 `use_dropdown_layer(open)` 基础上，抽象一个更通用的 `use_floating_layer(kind, open)`：
    - 入参：浮层 kind（或 role）+ open 布尔值；
    - 出参：
      - `z_index: Signal<i32>`：供浮层 UI 使用；
      - （可选）`key: Signal<Option<OverlayKey>>`：供未来需要直接操作 overlay entry 的场景使用；
    - 行为：
      - `open == true` 且当前无 key 时，调用 `overlay.open(kind, has_mask=false)` 登记新浮层；
      - `open == false` 且存在 key 时，调用 `overlay.close(key)` 关闭浮层；
      - 组件侧仅需根据 `open` 控制是否渲染具体 UI，z-index 统一由 OverlayManager 分配。

- 点击空白 / ESC 关闭的统一方案：
  - 现有 Select 家族采用：
    - 组件内维护 `internal_click_flag: Signal<bool>`；
    - 在触发区与下拉内部交互前将 flag 置为 true；
    - 在 `wasm32` 目标下注册 document 级 `click` 监听：
      - 若 flag 为 true，则重置 flag 并忽略本次事件；
      - 否则视为“外部点击”，关闭对应浮层；
    - 键盘 ESC 按键在组件内部处理，将 open 置为 false。
  - 计划复用这一模式，抽象为一个小工具（伪代码）：

    ```rust
    struct FloatingCloseHandle {
        pub internal_click_flag: Signal<bool>,
    }

    fn use_floating_close_handle(open: Signal<bool>) -> FloatingCloseHandle {
        let flag: Signal<bool> = use_signal(|| false);

        #[cfg(target_arch = "wasm32")]
        {
            let mut flag_for_global = flag;
            let mut open_for_global = open;
            use_effect(move || {
                use wasm_bindgen::{closure::Closure, JsCast};
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        let target: web_sys::EventTarget = document.into();
                        let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
                            move |_evt| {
                                let mut f = flag_for_global;
                                if *f.read() {
                                    f.set(false);
                                    return;
                                }
                                let mut o = open_for_global;
                                if *o.read() {
                                    o.set(false);
                                }
                            },
                        ));
                        let _ = target.add_event_listener_with_callback(
                            "click",
                            handler.as_ref().unchecked_ref(),
                        );
                        handler.forget();
                    }
                }
            });
        }

        FloatingCloseHandle { internal_click_flag: flag }
    }
    ```

  - Tooltip / Popover / Popconfirm / Dropdown 会统一使用该 helper：
    - 在触发区与浮层内部交互前，将 `internal_click_flag` 设为 true；
    - ESC 关闭逻辑在组件内部处理，保持行为一致。

- 触发与定位策略：
  - MVP 阶段采用简化定位：
    - Tooltip/Popover/Popconfirm：优先实现 `Top/Bottom/Left/Right` 四个方向，使用相对触发元素的简单偏移（类似当前 Select 的 `top: 100%; left: 0`）；
    - Dropdown：默认 `BottomLeft`，未来再扩展 `BottomRight`/`Top*` 等变体；
  - 触发行为：
    - Tooltip：默认 hover，支持 click；
    - Popover/Popconfirm/Dropdown：默认 click，后续可按需扩展 hover/contextMenu。    

> 以上规划将在 `plan/0008.md` 的后续任务中，通过 `use_floating_layer` 和 Tooltip/Popover/Popconfirm/Dropdown 的具体实现落地。

## 4. 导航与分页组件规划（Menu / Breadcrumb / Pagination）

在完成 Select 家族与 Tooltip/Popover/Popconfirm/Dropdown 之后，导航与分页组件是构建中后台应用骨架的下一组核心能力：

- `Menu`：主要作为 Layout.Sider 或 Layout.Header 内的导航结构，承载站点层级与当前选中态；
- `Breadcrumb`：用于展示当前页面在站点层级中的位置，常与路由状态或 Menu 选中状态对应；
- `Pagination`：主要服务于列表类视图（Table/List/Card），控制当前页与每页数量。

规划原则：

- Menu
  - 与 Layout 解耦：组件本身只关心数据（`items` / `selected_keys` / `open_keys`）与交互回调，不绑定具体路由实现；
  - 支持 Inline / Horizontal 两种模式，分别适配 Sider（侧边栏）与 Header（顶部导航）；
  - 通过受控/非受控 `selected_keys` / `open_keys`，让上层按需接入路由或全局状态。
- Breadcrumb
  - 视为纯展示组件：通过 `items: Vec<BreadcrumbItem>` 显式描述路径，而非隐式依赖路由容器；
  - 提供可选 `separator` 与点击回调，便于与路由或自定义导航交互；
  - 不负责实际跳转逻辑，由外层路由/应用代码处理。
- Pagination
  - 仅负责页码/页大小的状态与 UI，数据加载由业务层完成；
  - 采用受控 `current` / `page_size` 优先的模式，通过 `on_change` / `on_page_size_change` 将状态向外反馈；
  - 与 Select/Button 等复用主题 token，使分页器在视觉上与整体系统一致。

与 App / ConfigProvider 的关系：

- ConfigProvider
  - 通过 `ComponentSize` 影响 Menu/Pagination 的尺寸（如紧凑/默认/偏大），与 Button/Input 等保持一致；
  - 全局 `disabled` 不直接作用于 Menu/Breadcrumb/Pagination，但相关 demo 会遵循「业务禁用」语义（如某些 MenuItem 禁用）。
- App / OverlayManager
  - Menu/Breadcrumb/Pagination 不直接参与 Overlay 管理，只依赖已有 Layout/Theme 结构；
  - 与浮层组件（Dropdown/Tooltip 等）的组合由具体组件内部处理，App 层仅负责提供全局反馈能力（Message/Notification）。

以上规划将作为 `plan/0009.md` 中 Menu/Breadcrumb/Pagination 实现的约束与参考，并在对应组件文档中进一步细化。

## 5. 数据展示与加载组件规划（List / Table / Empty / Spin / Skeleton）

在导航与分页组件之后，下一步是补齐中后台中最常见的「数据展示 + 状态反馈」组件层，用于承载列表页和数据表格页：

- `List`：用于中轻量级数据展示，支持简单分页、空状态和加载态；
- `Table`：结构化数据表格，后续会逐步增强排序、筛选等能力；
- `Empty`：统一的“无数据”空状态，适配 List/Table/Select 等场景；
- `Spin`：通用加载指示器，适合包裹局部内容或整页 loading；
- `Skeleton`：骨架屏，占位式加载体验，比 Spin 更聚焦在“结构感”。

规划要点：

- 与 Pagination/Navigation 协同：
  - List/Table 优先通过受控 props 与 `Pagination` 协同，而不是在内部重新实现分页逻辑；
  - 配合 `Menu`/`Breadcrumb`/`Layout`，形成“导航 + 列表页”基础模板（例如：左侧 Menu + 顶部 Breadcrumb + 内容区 Table + Pagination）。
- 统一的 loading/empty 体验：
  - List/Table 在 `loading=true` 时优先通过 `Spin`/`Skeleton` 包裹内容，而非自定义 loading UI；
  - 当数据为空且非加载中时，默认渲染 `Empty`，并允许自定义说明文案或操作入口（如“创建第一个项目”按钮）。
- 受控优先：
  - List/Table 的分页/排序状态由外层业务管理，组件通过回调报告用户操作；
  - Empty/Spin/Skeleton 自身保持“傻组件”，不持久化业务状态，只负责视觉呈现。

与 App / ConfigProvider 的关系：

- ConfigProvider
  - 通过 `ComponentSize` 影响 List/Table 行高与间距，保持与 Button/Input 等一致的紧凑/默认/偏大风格；
  - 通过主题 token 控制 Empty/Spin/Skeleton 的颜色、圆角与动画节奏（例如暗色模式下的骨架底色）。
- App / OverlayManager
  - List/Table/Empty/Spin/Skeleton 本身不直接使用 OverlayManager，但经常与 Modal/Drawer/Message/Notification 组合：
    - 例如：表格行操作触发 Modal 编辑，成功后通过 Message 提示并刷新数据；
    - 整页加载态可通过 Spin/Skeleton 与 Notification 组合告知进度。

这些规划将通过 `plan/0010.md` 中的任务逐步落地，并在 `docs/list.md` / `docs/table.md` / `docs/empty.md` / `docs/spin.md` / `docs/skeleton.md` 中细化。内置示例 `examples/data_view_demo.rs` 进一步展示了 Layout + Breadcrumb + Table/List + Pagination + Empty/Spin/Skeleton 的组合，作为典型“用户列表页”模板参考。

## 6. 导航 + 卡片 + 标签/角标 组合规划（Tabs / Card / Tag / Badge / Avatar）

在完成列表/表格与加载/空状态组件后，下一步是补齐「导航 + 信息卡片 + 状态标签/角标」这一层，支撑常见的用户中心、仪表盘和设置页：

- `Tabs`：
  - 负责单页内视图切换，例如「基础信息 / 安全设置 / 通知偏好」；
  - 建议与 Layout/Header/Breadcrumb 组合，形成「页面级导航 + 页签级导航」；
- `Card`：
  - 作为中型内容容器，承载统计信息块、配置表单、列表卡片等；
  - 建议与 Grid/Masonry/List 组合，形成仪表盘式卡片布局；
- `Tag`：
  - 用于表达状态/标签/筛选条件，例如 Table 列中的“启用/停用”标记，或者列表项的分类标签；
- `Badge`：
  - 作为数量角标/小红点，挂在 Menu/Icon/Button/Avatar 上表达未读通知、待办数量等；
- `Avatar`：
  - 用于展示用户/应用头像，常与 Dropdown/Menu/Badge 组合构成顶部用户信息区域。

组合建议：

- 顶部导航区域：Layout.Header + Menu + Avatar(+Dropdown) + Badge，用于全局导航和用户信息；
- 内容区：Tabs + Card + List/Table/Empty/Spin/Skeleton，用于不同视图下的列表/表单/统计卡片；
- 状态展示：在 Table/List 行内使用 Tag 表达状态（成功/警告/错误/处理中），在按钮/图标上使用 Badge 表达数量。

这些组件的迁移与实现将在 `plan/0011.md` 中逐步落地，对应文档为 `docs/tabs.md` / `docs/card.md` / `docs/tag.md` / `docs/badge.md` / `docs/avatar.md`。

## 7. 流程与结果反馈规划（Alert / Result / Progress / Statistic / Steps）

在导航、列表与卡片体系完善之后，还需要一层面向「流程与结果反馈」的组件，用于串联多步操作、任务执行进度与最终结果页面：

- `Alert`：页面内的重要提示与警告，用于承载表单顶部告警、权限提示、全局配置提醒等；
- `Result`：操作结果页的主承载区域，适合放在独立路由页面或 Card 中，配合按钮组给出后续动作；
- `Progress`：用于展示中长任务（上传、批量处理、后台任务）的完成度，可与 Steps/Result 组合；
- `Statistic`：关键指标数值展示（如总访问量、转化率、错误率），常与 Card/Grid 组合形成统计面板；
- `Steps`：流程步骤条，用于引导多步表单、审批流或配置向导等复杂流程。

组合建议：

- 多步表单：使用 Steps 控制当前步骤，Form 负责每步内容，配合 Alert 展示校验/权限提示，完成后跳转 Result 页面；
- 长任务：使用 Progress 展示执行进度，过程中通过 Message/Notification 提示阶段结果，失败时在页内使用 Alert 或 Result 告知原因；
- 仪表盘：在 Card 中组合 Statistic + Progress + Tag/Badge，展示整体健康度与关键指标，并配合 Layout/Tabs 组织不同视图。

这些组件的迁移与实现将在 `plan/0012.md` 中逐步落地，对应文档为 `docs/alert.md` / `docs/result.md` / `docs/progress.md` / `docs/statistic.md` / `docs/steps.md`。

## 8. 日期时间组件规划（DatePicker / TimePicker / Calendar）

在流程与结果反馈层之上，还需要补齐「时间维度」相关的输入与视图组件，用于表达业务数据的时间轴：

- `DatePicker`：
  - 承担单个日期选择与日期区间选择（RangePicker）的职责，是表单中最常见的日期输入控件；
  - 与 Form 深度集成，用于下单时间、有效期、统计区间等场景；
- `TimePicker`：
  - 负责时间点选择，支持时/分/秒步进，常与 DatePicker 组合成日期时间选择；
  - 常见于预约时间、定时任务、通知发送时间等场景；
- `Calendar`：
  - 提供整月/全年视图的日期展示，承载日程、排班等信息；
  - 一般与 Badge / Tag / Tooltip 等组合展示每日事件状态。

规划要点：

- 统一内部日期时间模型：优先选用稳定的 Rust 日期时间库（如 `time`），对外通过统一的值表示（例如字符串或内部枚举封装），避免业务侧直接依赖第三方库细节；
- 与 Form：
  - DatePicker / TimePicker / RangePicker 在 Form 中视为受控字段，支持校验规则（必填、开始时间早于结束时间、不能早于今天等）；
  - Form 的重置/提交逻辑需要与日期时间组件的内部状态保持完全一致；
- 与 ConfigProvider / Locale：
  - 日期文案（星期、月份、"今天"、"此刻" 等）从 ConfigProvider 的 locale 中统一获取，支持中英文切换；
  - 尺寸与禁用状态与现有 Input / Select 保持一致；
- 与 Overlay / Select 家族：
  - DatePicker / TimePicker 的面板浮层复用 Select / Cascader 已有的下拉机制（OverlayManager + click-outside + ESC 关闭）；
  - Calendar 作为常驻视图组件，优先与 Layout / Card / Badge 家族组合，构成「日程 + 列表/详情」的典型页面模板。

这组组件的实现与迁移任务将集中在 `plan/0013.md` 中推进，对应文档为 `docs/date_picker.md` / `docs/time_picker.md` / `docs/calendar.md`（落地后补充）。

## 9. App 与 ConfigProvider 的协同能力

- （以下内容保持原有结构，仅略微调整标题编号，未改变语义）

### App 与 use_* API

- 使用方式：
  - 在应用根部包裹：`ConfigProvider { App { .. } }`（或先 `App` 再内部自己挂 ConfigProvider）；
  - 在任意子组件中：
    - `let msg = use_message();`
    - `let notice = use_notification();`
    - `let modal = use_modal();`。
- 设计要点：
  - `App` 内部通过 `use_overlay_provider()` 安装一个 OverlayManager，确保所有全局反馈组件共享同一套 z-index 管理；
  - `use_app()` 返回完整的 `AppContextValue`，`use_message` / `use_notification` / `use_modal` 只是便捷封装；
  - 当前阶段 Message/Notification/Modal 的 API 是 MVP 占位实现（例如 `open()` 仅登记 overlay 层级），视觉与完整行为在后续小节实现。

### 4.2 ConfigProvider 与 ThemeProvider

- `ConfigProvider` 当前支持：
  - `size`: small / middle / large（映射到 `ComponentSize`），用于控制 Button 等组件的默认尺寸；
  - `disabled`: 全局禁用标记，Input 等组件会在自身 `disabled` 为 false 时仍遵守全局禁用；
  - `prefix_cls`: 目前仅在样式命名上预留，对现有 `adui-` 前缀保持兼容。
- 优先级规则：
  - 组件本地 props（如 `ButtonProps.size`、`InputProps.disabled`）优先于全局配置；
  - `ButtonGroup` 中的 size/shape 优先级高于 ConfigProvider 的全局 size；
  - Form 内部的 `disabled` 与 ConfigProvider 的 `disabled` 组合生效，任一为 true 即禁用。

- 与 `ThemeProvider`：
  - ConfigProvider 会优先复用现有 ThemeProvider 中的 token 和主题定义，避免重复配置；
  - ThemeProvider 仍然是底层“主题来源”，ConfigProvider 侧重于 UI 语义配置（size、disabled、prefix、locale 等）。

- 与 Form / Input 等：
  - Modal / Drawer 的设计会优先满足「弹窗表单」场景，保持与当前 Form 验证模型兼容；
  - Message / Notification 将作为 Form 提交结果的主要反馈方式之一，在文档示例中重点展示。

## 6. 全局反馈：Message / Notification

（以下章节与原文一致，略）

## 6. 浮层组件：Modal / Drawer

（保持原有内容）

## 7. 选择器家族（规划）

（保持原有内容，仅在 3.2 中补充了浮层规划引用）

## 8. 后续可扩展方向（非本轮目标）

（保持原有内容）
