use crate::components::grid::ColProps;
use crate::foundation::{
    ClassListExt, FormClassNames, FormSemantic, FormStyles, StyleStringExt, Variant,
};
use dioxus::{
    core::{current_scope_id, schedule_update_any},
    prelude::*,
};
use regex::Regex;
use serde_json::Value;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

pub type FormValues = HashMap<String, Value>;
pub type FormErrors = HashMap<String, String>;
pub type FormValidator = fn(Option<&Value>) -> Result<(), String>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormLayout {
    #[default]
    Horizontal,
    Vertical,
    Inline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ControlSize {
    Small,
    #[default]
    Middle,
    Large,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RequiredMark {
    None,
    Optional,
    #[default]
    Default,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelAlign {
    Left,
    #[default]
    Right,
}

/// Configuration for feedback icons displayed in form items.
///
/// When enabled, shows success/error/warning/validating icons in form items.
#[derive(Clone, Default, PartialEq)]
pub struct FeedbackIcons {
    /// Custom success icon element.
    pub success: Option<Element>,
    /// Custom error icon element.
    pub error: Option<Element>,
    /// Custom warning icon element.
    pub warning: Option<Element>,
    /// Custom validating icon element.
    pub validating: Option<Element>,
}

impl FeedbackIcons {
    /// Create feedback icons with default icons.
    pub fn default_icons() -> Self {
        Self {
            success: None, // Will use default Icon component
            error: None,
            warning: None,
            validating: None,
        }
    }
}

/// Configuration for scroll behavior when validation fails.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScrollToFirstErrorConfig {
    /// Block position for scrollIntoView.
    pub block: Option<String>,
    /// Inline position for scrollIntoView.
    pub inline: Option<String>,
    /// Behavior for scrollIntoView (smooth/instant/auto).
    pub behavior: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct FormRule {
    pub required: bool,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub len: Option<usize>,
    pub pattern: Option<String>,
    pub message: Option<String>,
    pub validator: Option<FormValidator>,
}

impl PartialEq for FormRule {
    fn eq(&self, other: &Self) -> bool {
        self.required == other.required
            && self.min == other.min
            && self.max == other.max
            && self.len == other.len
            && self.pattern == other.pattern
            && self.message == other.message
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormFinishEvent {
    pub values: FormValues,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormFinishFailedEvent {
    pub errors: FormErrors,
}

/// Event payload describing value changes at the Form level.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValuesChangeEvent {
    /// This update中实际发生变化的字段集合（通常只包含当前字段）。
    pub changed_values: FormValues,
    /// 变更后的完整字段快照。
    pub all_values: FormValues,
}

#[derive(Clone)]
struct FormStore {
    values: FormValues,
    errors: FormErrors,
}

impl FormStore {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            errors: HashMap::new(),
        }
    }
}

static FORM_HANDLE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct FormHandle {
    store: Rc<RefCell<FormStore>>,
    listeners: Rc<RefCell<HashMap<String, Vec<ScopeId>>>>,
    id: usize,
}

impl PartialEq for FormHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FormHandle {
    pub fn new() -> Self {
        let id = FORM_HANDLE_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            store: Rc::new(RefCell::new(FormStore::new())),
            listeners: Rc::new(RefCell::new(HashMap::new())),
            id,
        }
    }

    pub fn set_field_value(&self, name: &str, value: Value) {
        if let Ok(mut store) = self.store.try_borrow_mut() {
            store.values.insert(name.to_string(), value);
        }
        self.notify(name);
    }

    pub fn get_field_value(&self, name: &str) -> Option<Value> {
        self.store
            .try_borrow()
            .ok()
            .and_then(|store| store.values.get(name).cloned())
    }

    pub fn set_error(&self, name: &str, message: Option<String>) {
        if let Ok(mut store) = self.store.try_borrow_mut() {
            match message {
                Some(msg) => {
                    store.errors.insert(name.to_string(), msg);
                }
                None => {
                    store.errors.remove(name);
                }
            }
        }
    }

    pub fn get_error(&self, name: &str) -> Option<String> {
        self.store
            .try_borrow()
            .ok()
            .and_then(|store| store.errors.get(name).cloned())
    }

    pub fn values(&self) -> FormValues {
        self.store
            .try_borrow()
            .map(|store| store.values.clone())
            .unwrap_or_default()
    }

    pub fn errors(&self) -> FormErrors {
        self.store
            .try_borrow()
            .map(|store| store.errors.clone())
            .unwrap_or_default()
    }

    pub fn reset_fields(&self) {
        if let Ok(mut store) = self.store.try_borrow_mut() {
            store.values.clear();
            store.errors.clear();
        }
        let names = self.listeners.borrow().keys().cloned().collect::<Vec<_>>();
        self.notify_all(&names);
    }

    fn notify(&self, name: &str) {
        let targets = self
            .listeners
            .borrow()
            .get(name)
            .cloned()
            .unwrap_or_default();
        let scheduler = schedule_update_any();
        for scope in targets {
            scheduler(scope);
        }
    }

    fn notify_all(&self, names: &[String]) {
        for name in names {
            self.notify(name);
        }
    }

    pub fn register_listener(&self, name: &str, scope_id: ScopeId) {
        let mut map = self.listeners.borrow_mut();
        let entry = map.entry(name.to_string()).or_default();
        if !entry.contains(&scope_id) {
            entry.push(scope_id);
        }
    }
}

impl Default for FormHandle {
    fn default() -> Self {
        FormHandle::new()
    }
}

#[derive(Clone)]
struct FormContext {
    handle: FormHandle,
    _layout: FormLayout,
    _size: ControlSize,
    colon: bool,
    required_mark: RequiredMark,
    _label_align: LabelAlign,
    _label_col: Option<ColProps>,
    _wrapper_col: Option<ColProps>,
    disabled: bool,
    variant: Option<Variant>,
    feedback_icons: Option<FeedbackIcons>,
    registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
    on_values_change: Option<EventHandler<ValuesChangeEvent>>,
}

#[derive(Clone)]
pub struct FormItemControlContext {
    name: String,
    handle: FormHandle,
    disabled: bool,
    registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
    on_values_change: Option<EventHandler<ValuesChangeEvent>>,
    value_prop_name: Option<String>,
    get_value_from_event: Option<GetValueFromEventFn>,
}

impl FormItemControlContext {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> Option<Value> {
        self.handle.get_field_value(&self.name)
    }

    /// Optional metadata describing which prop should be treated as value
    /// when integrating with custom controls (similar to AntD's valuePropName).
    pub fn value_prop_name(&self) -> Option<&str> {
        self.value_prop_name.as_deref()
    }

    /// Apply a raw `Value` coming from a custom event, passing it through
    /// `get_value_from_event` if configured, then writing it into FormStore.
    pub fn apply_mapped_value(&self, raw: Value) {
        let mapped = if let Some(mapper) = self.get_value_from_event {
            mapper(raw)
        } else {
            raw
        };
        self.set_value(mapped);
    }

    pub fn set_value(&self, value: Value) {
        if self.disabled {
            return;
        }
        self.handle.set_field_value(&self.name, value);
        self.run_validation();

        if let Some(cb) = self.on_values_change.as_ref() {
            let mut changed = FormValues::new();
            if let Some(current) = self.handle.get_field_value(&self.name) {
                changed.insert(self.name.clone(), current);
            }
            let all = self.handle.values();
            cb.call(ValuesChangeEvent {
                changed_values: changed,
                all_values: all,
            });
        }
    }

    pub fn set_string(&self, value: impl Into<String>) {
        self.set_value(Value::String(value.into()));
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn run_validation(&self) {
        apply_validation_result(&self.handle, &self.registry, &self.name);
    }
}

/// Context exposed by `FormList`, providing access to the parent list field
/// and the underlying `FormHandle`.
#[derive(Clone)]
pub struct FormListContext {
    pub name: String,
    pub handle: FormHandle,
}

impl FormListContext {
    /// Current length of the list field.
    pub fn len(&self) -> usize {
        form_list_len(&self.handle, &self.name)
    }

    /// Whether the list field is currently empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert an item at the given index.
    pub fn insert(&self, index: usize, item: Value) {
        form_list_insert(&self.handle, &self.name, index, item);
    }

    /// Remove an item at the given index.
    pub fn remove(&self, index: usize) {
        form_list_remove(&self.handle, &self.name, index);
    }
}

/// Access the nearest `FormListContext`, if any.
pub fn use_form_list() -> Option<FormListContext> {
    try_use_context::<FormListContext>()
}

pub fn use_form_item_control() -> Option<FormItemControlContext> {
    if let Some(ctx) = try_use_context::<FormItemControlContext>() {
        // Register the current scope as a listener for this field so that
        // changes in the FormStore (set_field_value/reset_fields) will
        // trigger a re-render of the component using this context.
        let scope = current_scope_id();
        ctx.handle.register_listener(ctx.name(), scope);
        Some(ctx)
    } else {
        None
    }
}

/// Create a standalone form handle (类似 `Form.useForm`).
pub fn use_form() -> FormHandle {
    FormHandle::new()
}

#[derive(Props, Clone, PartialEq)]
pub struct FormProps {
    #[props(default)]
    pub layout: FormLayout,
    #[props(default)]
    pub size: ControlSize,
    #[props(default)]
    pub colon: bool,
    #[props(default)]
    pub required_mark: RequiredMark,
    #[props(default)]
    pub label_align: LabelAlign,
    #[props(optional)]
    pub label_col: Option<ColProps>,
    #[props(optional)]
    pub wrapper_col: Option<ColProps>,
    #[props(default)]
    pub disabled: bool,
    /// Visual variant for form controls (outlined/filled/borderless).
    #[props(optional)]
    pub variant: Option<Variant>,
    /// Whether to scroll to the first field with error on submit failure.
    /// Can be true (default scroll behavior) or a config object.
    #[props(default)]
    pub scroll_to_first_error: bool,
    /// Custom scroll config for scroll_to_first_error.
    #[props(optional)]
    pub scroll_to_first_error_config: Option<ScrollToFirstErrorConfig>,
    /// Feedback icons configuration for form items.
    #[props(optional)]
    pub feedback_icons: Option<FeedbackIcons>,
    #[props(optional)]
    pub initial_values: Option<FormValues>,
    #[props(optional)]
    pub form: Option<FormHandle>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names for sub-parts.
    #[props(optional)]
    pub class_names: Option<FormClassNames>,
    /// Semantic styles for sub-parts.
    #[props(optional)]
    pub styles: Option<FormStyles>,
    #[props(optional)]
    pub on_finish: Option<EventHandler<FormFinishEvent>>,
    #[props(optional)]
    pub on_finish_failed: Option<EventHandler<FormFinishFailedEvent>>,
    /// 表单字段值变更时的回调，主要用于字段联动与调试。
    #[props(optional)]
    pub on_values_change: Option<EventHandler<ValuesChangeEvent>>,
    pub children: Element,
}

#[component]
pub fn Form(props: FormProps) -> Element {
    let FormProps {
        layout,
        size,
        colon,
        required_mark,
        label_align,
        label_col,
        wrapper_col,
        disabled,
        variant,
        scroll_to_first_error,
        scroll_to_first_error_config,
        feedback_icons,
        initial_values,
        form,
        class,
        style,
        class_names,
        styles,
        on_finish,
        on_finish_failed,
        on_values_change,
        children,
    } = props;

    let handle = form.unwrap_or_else(FormHandle::new);
    if let Some(initial) = initial_values
        && handle.values().is_empty()
    {
        for (k, v) in initial.into_iter() {
            handle.set_field_value(&k, v);
        }
    }

    // Registry 需要在整个 Form 生命周期内保持稳定的引用，不能每次渲染都重新创建。
    // 使用 signal 保证只在首次渲染时构建一次 Rc<RefCell<..>>。
    let registry_signal = use_signal::<Rc<RefCell<HashMap<String, Vec<FormRule>>>>>(|| {
        Rc::new(RefCell::new(HashMap::new()))
    });
    let registry = registry_signal.read().clone();
    let context = FormContext {
        handle: handle.clone(),
        _layout: layout,
        _size: size,
        colon,
        required_mark,
        _label_align: label_align,
        _label_col: label_col,
        _wrapper_col: wrapper_col,
        disabled,
        variant,
        feedback_icons,
        registry: registry.clone(),
        on_values_change,
    };
    use_context_provider(|| context);

    let submit_handle = handle.clone();
    let submit_registry = registry.clone();
    let finish_cb = on_finish;
    let failed_cb = on_finish_failed;

    // Build form classes
    let mut form_classes = vec![form_class(layout, size)];
    form_classes.push_semantic(&class_names, FormSemantic::Root);
    if let Some(extra) = class {
        form_classes.push(extra);
    }
    let form_class_attr = form_classes.into_iter().filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ");

    let mut form_style_attr = style.unwrap_or_default();
    form_style_attr.append_semantic(&styles, FormSemantic::Root);

    // Store scroll config for use in submit handler
    let _scroll_config = scroll_to_first_error_config;

    rsx! {
        form {
            class: "{form_class_attr}",
            style: "{form_style_attr}",
            onsubmit: move |evt| {
                evt.prevent_default();
                if validate_all(&submit_handle, &submit_registry) {
                    if let Some(cb) = finish_cb.as_ref() {
                        cb.call(FormFinishEvent { values: submit_handle.values() });
                    }
                } else {
                    // Scroll to first error if enabled
                    if scroll_to_first_error {
                        #[cfg(target_arch = "wasm32")]
                        {
                            scroll_to_first_error_field(&submit_handle, &submit_registry);
                        }
                    }
                    if let Some(cb) = failed_cb.as_ref() {
                        cb.call(FormFinishFailedEvent { errors: submit_handle.errors() });
                    }
                }
            },
            {children}
        }
    }
}

/// Scroll to the first field with an error (WASM only).
#[cfg(target_arch = "wasm32")]
fn scroll_to_first_error_field(
    handle: &FormHandle,
    registry: &Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
) {
    let errors = handle.errors();
    if errors.is_empty() {
        return;
    }

    // Get field names in registration order
    let field_names: Vec<String> = match registry.try_borrow() {
        Ok(map) => map.keys().cloned().collect(),
        Err(_) => return,
    };

    // Find first field with error
    for name in field_names {
        if errors.contains_key(&name) {
            // Try to find and scroll to the form item
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // Try to find by name attribute or data-field attribute
                    if let Some(element) = document
                        .query_selector(&format!("[name=\"{}\"]", name))
                        .ok()
                        .flatten()
                    {
                        let _ = element.scroll_into_view();
                    }
                }
            }
            break;
        }
    }
}

fn form_class(layout: FormLayout, size: ControlSize) -> String {
    let mut classes = vec!["adui-form".to_string()];
    match layout {
        FormLayout::Horizontal => classes.push("adui-form-horizontal".into()),
        FormLayout::Vertical => classes.push("adui-form-vertical".into()),
        FormLayout::Inline => classes.push("adui-form-inline".into()),
    }
    match size {
        ControlSize::Small => classes.push("adui-form-small".into()),
        ControlSize::Large => classes.push("adui-form-large".into()),
        ControlSize::Middle => {}
    }
    classes.join(" ")
}

fn validate_all(
    handle: &FormHandle,
    registry: &Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
) -> bool {
    // Snapshot 所有字段名，若无法读取 registry（理论上不应该发生），则保守返回失败。
    let names: Vec<String> = match registry.try_borrow() {
        Ok(map) => map.keys().cloned().collect(),
        Err(_) => {
            // 不冒险在无法读取规则时放行提交。
            return false;
        }
    };

    // 如果存在 required 规则且当前表单值整体为空，
    // 对所有带 required 的字段应用校验，并直接视为失败。
    // 这是专门覆盖 reset 后尚未输入任何值就立刻提交的场景。
    if handle.values().is_empty() {
        let mut required_names = Vec::new();
        let mut has_required = false;

        if let Ok(map) = registry.try_borrow() {
            for (name, rules) in map.iter() {
                if rules.iter().any(|rule| rule.required) {
                    has_required = true;
                    required_names.push(name.clone());
                }
            }
        } else {
            // 无法读取规则时也不放行提交。
            return false;
        }

        if has_required {
            for name in &required_names {
                apply_validation_result(handle, registry, name);
            }
            return false;
        }
    }

    let mut ok = true;
    for name in &names {
        if !apply_validation_result(handle, registry, name) {
            ok = false;
        }
    }

    ok
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormListItemMeta {
    pub index: usize,
    /// 完整字段名，例如 "users[0]"，供嵌套 FormItem 使用。
    pub name: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct FormListProps {
    /// 列表字段名，如 "users"。
    pub name: String,
    /// 可选的初始项数量，仅在 Form 中当前没有该字段值时生效。
    #[props(optional)]
    pub initial_count: Option<usize>,
    /// 列表内部渲染的子节点，由使用方自行遍历索引并渲染 FormItem。
    pub children: Element,
}

/// 占位实现：当前仅作为 API 声明，行为等同于普通 div 包裹 children。
/// 后续 B3 步骤会补充动态增删项逻辑。
#[component]
pub fn FormList(props: FormListProps) -> Element {
    let ctx = use_context::<FormContext>();
    let FormListProps {
        name,
        initial_count,
        children,
    } = props;

    // 若配置了 initial_count，且当前列表为空，则初始化指定数量的空元素。
    if let Some(count) = initial_count
        && count > 0
        && form_list_len(&ctx.handle, &name) == 0
    {
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(Value::Null);
        }
        form_list_set(&ctx.handle, &name, items);
    }

    let list_ctx = FormListContext {
        name: name.clone(),
        handle: ctx.handle.clone(),
    };
    use_context_provider(|| list_ctx);

    rsx! { div { class: "adui-form-list", {children} } }
}

pub type GetValueFromEventFn = fn(Value) -> Value;

// Function pointer only used for props equality in diffing; address-based
// comparison is acceptable for this narrow use case.
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Props, Clone, PartialEq)]
pub struct FormItemProps {
    #[props(optional)]
    pub name: Option<String>,
    #[props(optional)]
    pub label: Option<String>,
    #[props(optional)]
    pub tooltip: Option<String>,
    #[props(optional)]
    pub help: Option<String>,
    #[props(optional)]
    pub extra: Option<String>,
    #[props(optional)]
    pub rules: Option<Vec<FormRule>>,
    /// 简化版 dependencies：当前 FormItem 依赖的其他字段名列表，用于渲染联动。
    #[props(optional)]
    pub dependencies: Option<Vec<String>>,
    /// 自定义值映射：指定控件使用哪个 prop 作为值（类似 AntD 的 valuePropName）。
    #[props(optional)]
    pub value_prop_name: Option<String>,
    /// 自定义值映射：从原始 Value 映射到写入 FormStore 的 Value，供自定义控件配合 `apply_mapped_value` 使用。
    #[props(optional)]
    pub get_value_from_event: Option<GetValueFromEventFn>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(default)]
    pub has_feedback: bool,
    pub children: Element,
}

#[component]
pub fn FormItem(props: FormItemProps) -> Element {
    let ctx = use_context::<FormContext>();
    let FormItemProps {
        name,
        label,
        tooltip,
        help,
        extra,
        rules,
        dependencies,
        value_prop_name,
        get_value_from_event,
        class,
        style,
        has_feedback,
        children,
    } = props;

    let item_rules = rules.unwrap_or_default();
    let is_required = item_rules.iter().any(|rule| rule.required);

    let item_scope = current_scope_id();

    if let Some(field_name) = name.clone() {
        {
            ctx.registry
                .borrow_mut()
                .insert(field_name.clone(), item_rules.clone());
        }
        ctx.handle.register_listener(&field_name, item_scope);
        let control_ctx = FormItemControlContext {
            name: field_name.clone(),
            handle: ctx.handle.clone(),
            disabled: ctx.disabled,
            registry: ctx.registry.clone(),
            on_values_change: ctx.on_values_change,
            value_prop_name: value_prop_name.clone(),
            get_value_from_event,
        };
        use_context_provider(|| control_ctx);
        if !item_rules.is_empty() {
            let handle = ctx.handle.clone();
            let registry = ctx.registry.clone();
            use_effect(move || {
                apply_validation_result(&handle, &registry, &field_name);
            });
        }
    }

    // 简化版 dependencies：当依赖字段更新时，当前 FormItem scope 会被重新调度渲染。
    if let Some(deps) = dependencies.as_ref() {
        for dep in deps {
            ctx.handle.register_listener(dep, item_scope);
        }
    }

    let error_message = name.as_ref().and_then(|field| ctx.handle.get_error(field));

    let mut wrapper_class = vec!["adui-form-item".to_string()];
    if let Some(extra) = class {
        wrapper_class.push(extra);
    }
    if error_message.is_some() {
        wrapper_class.push("adui-form-item-has-error".into());
    } else if has_feedback {
        wrapper_class.push("adui-form-item-has-feedback".into());
    }

    let tooltip_text = tooltip.clone().unwrap_or_default();

    rsx! {
        div { class: wrapper_class.join(" "), style: style.unwrap_or_default(),
            if let Some(text) = label {
                label { class: "adui-form-item-label", title: "{tooltip_text}",
                    span {
                        if matches!(ctx.required_mark, RequiredMark::Default) && is_required {
                            span { class: "adui-form-item-required", "*" }
                        }
                        if ctx.colon {
                            "{text}:"
                        } else {
                            "{text}"
                        }
                    }
                }
            }
            div { class: "adui-form-item-control", {children} }
            if let Some(help_text) = error_message.or(help) {
                div { class: "adui-form-item-help", "{help_text}" }
            }
            if let Some(extra_text) = extra {
                div { class: "adui-form-item-extra", "{extra_text}" }
            }
        }
    }
}

fn apply_validation_result(
    handle: &FormHandle,
    registry: &Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
    name: &str,
) -> bool {
    if let Some(err) = validate_field(handle, registry, name) {
        handle.set_error(name, Some(err));
        false
    } else {
        handle.set_error(name, None);
        true
    }
}

fn validate_field(
    handle: &FormHandle,
    registry: &Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
    name: &str,
) -> Option<String> {
    let rules = registry
        .try_borrow()
        .ok()
        .and_then(|map| map.get(name).cloned())
        .unwrap_or_default();
    run_rules(handle, name, &rules)
}

fn run_rules(handle: &FormHandle, name: &str, rules: &[FormRule]) -> Option<String> {
    if rules.is_empty() {
        return None;
    }
    let value = handle.get_field_value(name);
    for rule in rules {
        if let Some(msg) = evaluate_rule(rule, value.as_ref()) {
            return Some(msg);
        }
    }
    None
}

fn evaluate_rule(rule: &FormRule, value: Option<&Value>) -> Option<String> {
    if rule.required && value_is_empty(value) {
        return Some(
            rule.message
                .clone()
                .unwrap_or_else(|| "必填项不能为空".into()),
        );
    }

    if value_is_empty(value) {
        return None;
    }

    if let Some(len_target) = rule.len
        && value_length(value) != Some(len_target)
    {
        return Some(
            rule.message
                .clone()
                .unwrap_or_else(|| format!("长度必须为 {}", len_target)),
        );
    }

    if let Some(min) = rule.min {
        match (value_length(value), numeric_value(value)) {
            (Some(len), _) if len < min => {
                return Some(
                    rule.message
                        .clone()
                        .unwrap_or_else(|| format!("长度不能小于 {}", min)),
                );
            }
            (_, Some(num)) if num < min as f64 => {
                return Some(
                    rule.message
                        .clone()
                        .unwrap_or_else(|| format!("数值不能小于 {}", min)),
                );
            }
            _ => {}
        }
    }

    if let Some(max) = rule.max {
        match (value_length(value), numeric_value(value)) {
            (Some(len), _) if len > max => {
                return Some(
                    rule.message
                        .clone()
                        .unwrap_or_else(|| format!("长度不能大于 {}", max)),
                );
            }
            (_, Some(num)) if num > max as f64 => {
                return Some(
                    rule.message
                        .clone()
                        .unwrap_or_else(|| format!("数值不能大于 {}", max)),
                );
            }
            _ => {}
        }
    }

    if let Some(pattern) = &rule.pattern
        && let Some(text) = value_to_string(value)
    {
        match Regex::new(pattern) {
            Ok(re) => {
                if !re.is_match(&text) {
                    return Some(rule.message.clone().unwrap_or_else(|| "格式不匹配".into()));
                }
            }
            Err(_) => {
                return Some(format!("无效的正则表达式: {}", pattern));
            }
        }
    }

    if let Some(validator) = rule.validator
        && let Err(err) = validator(value)
    {
        return Some(err);
    }

    None
}

fn value_is_empty(value: Option<&Value>) -> bool {
    match value {
        None => true,
        Some(Value::Null) => true,
        Some(Value::String(text)) => text.trim().is_empty(),
        Some(Value::Array(list)) => list.is_empty(),
        Some(Value::Object(map)) => map.is_empty(),
        // 布尔值视 false 为“空”，便于在 Checkbox / Switch 等场景下用 required 表示“必须为 true”
        Some(Value::Bool(flag)) => !flag,
        _ => false,
    }
}

fn value_length(value: Option<&Value>) -> Option<usize> {
    value.and_then(|val| match val {
        Value::String(text) => Some(text.chars().count()),
        Value::Array(list) => Some(list.len()),
        Value::Object(map) => Some(map.len()),
        _ => None,
    })
}

fn numeric_value(value: Option<&Value>) -> Option<f64> {
    value.and_then(|val| match val {
        Value::Number(num) => num.as_f64(),
        _ => None,
    })
}

fn value_to_string(value: Option<&Value>) -> Option<String> {
    value.and_then(|val| match val {
        Value::String(text) => Some(text.clone()),
        Value::Number(num) => Some(num.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Null => None,
        Value::Array(_) | Value::Object(_) => None,
    })
}

// ---- Shared helpers for mapping serde_json::Value to primitive types ----

/// Convert an optional `Value` into a `String` suitable for text inputs.
///
/// - `Null`/`None` → empty string
/// - `String` → itself
/// - `Number`/`Bool` → `to_string()`
/// - `Array`/`Object` → empty string
pub fn form_value_to_string(val: Option<Value>) -> String {
    match val {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s,
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => {
            if b {
                "true".into()
            } else {
                "false".into()
            }
        }
        Some(Value::Array(_)) | Some(Value::Object(_)) => String::new(),
    }
}

/// Convert an optional `Value` into a boolean, falling back to `default` when
/// no useful information is present.
pub fn form_value_to_bool(val: Option<Value>, default: bool) -> bool {
    match val {
        Some(Value::Bool(b)) => b,
        Some(Value::Number(n)) => n.as_f64().map(|v| v != 0.0).unwrap_or(default),
        Some(Value::String(s)) => match s.as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => default,
        },
        _ => default,
    }
}

/// Convert an optional `Value` into a vector of strings (used by CheckboxGroup).
/// Non-string entries in the array are ignored.
pub fn form_value_to_string_vec(val: Option<Value>) -> Vec<String> {
    match val {
        Some(Value::Array(items)) => items
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

/// Convert an optional `Value` into a radio key string.
/// Accepts strings, numbers and booleans and normalises them to `String`.
pub fn form_value_to_radio_key(val: Option<Value>) -> Option<String> {
    match val {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(s),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Bool(b)) => Some(if b { "true".into() } else { "false".into() }),
        Some(Value::Array(_)) | Some(Value::Object(_)) => None,
    }
}

/// Helper: read the list value for a given field as a `Vec<Value>`.
///
/// If the field is missing or not an array, returns an empty vector.
pub fn form_list_get(handle: &FormHandle, name: &str) -> Vec<Value> {
    match handle.get_field_value(name) {
        Some(Value::Array(items)) => items,
        _ => Vec::new(),
    }
}

/// Helper: set the list value for a given field from a `Vec<Value>`.
pub fn form_list_set(handle: &FormHandle, name: &str, items: Vec<Value>) {
    handle.set_field_value(name, Value::Array(items));
}

/// Helper: return the current length of a list field.
pub fn form_list_len(handle: &FormHandle, name: &str) -> usize {
    form_list_get(handle, name).len()
}

/// Helper: insert an item into a list field at the given index.
///
/// If the index is greater than the current length, the item is appended.
pub fn form_list_insert(handle: &FormHandle, name: &str, index: usize, item: Value) {
    let mut items = form_list_get(handle, name);
    let idx = index.min(items.len());
    items.insert(idx, item);
    form_list_set(handle, name, items);
}

/// Helper: remove an item from a list field at the given index.
///
/// If the index is out of bounds, this is a no-op.
pub fn form_list_remove(handle: &FormHandle, name: &str, index: usize) {
    let mut items = form_list_get(handle, name);
    if index < items.len() {
        items.remove(index);
        form_list_set(handle, name, items);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_value_to_string_covers_common_variants() {
        assert_eq!(form_value_to_string(None), "");
        assert_eq!(form_value_to_string(Some(Value::Null)), "");
        assert_eq!(
            form_value_to_string(Some(Value::String("abc".into()))),
            "abc"
        );
        assert_eq!(form_value_to_string(Some(Value::Number(42.into()))), "42");
        assert_eq!(form_value_to_string(Some(Value::Bool(true))), "true");
        assert_eq!(form_value_to_string(Some(Value::Bool(false))), "false");
    }

    #[test]
    fn form_value_to_bool_falls_back_to_default() {
        assert!(!form_value_to_bool(None, false));
        assert!(form_value_to_bool(None, true));
        assert!(form_value_to_bool(Some(Value::Bool(true)), false));
        assert!(!form_value_to_bool(Some(Value::Bool(false)), true));
        assert!(form_value_to_bool(Some(Value::Number(1.into())), false));
        assert!(!form_value_to_bool(Some(Value::Number(0.into())), true));
        assert!(form_value_to_bool(
            Some(Value::String("true".into())),
            false
        ));
        assert!(!form_value_to_bool(
            Some(Value::String("false".into())),
            true
        ));
    }

    #[test]
    fn form_value_to_string_vec_extracts_strings() {
        let val = Value::Array(vec![
            Value::String("a".into()),
            Value::Number(1.into()),
            Value::String("b".into()),
        ]);
        let vec = form_value_to_string_vec(Some(val));
        assert_eq!(vec, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn form_value_to_radio_key_converts_basic_types() {
        assert_eq!(form_value_to_radio_key(None), None);
        assert_eq!(
            form_value_to_radio_key(Some(Value::String("x".into()))),
            Some("x".into())
        );
        assert_eq!(
            form_value_to_radio_key(Some(Value::Number(1.into()))),
            Some("1".into())
        );
        assert_eq!(
            form_value_to_radio_key(Some(Value::Bool(true))),
            Some("true".into())
        );
    }

    // This helper exercises list operations on top of FormStore. It requires
    // the Dioxus runtime because `set_field_value` schedules updates, so we
    // keep it ignored in the default test run.
    #[test]
    #[ignore]
    fn form_list_helpers_operate_on_value_array() {
        let handle = FormHandle::new();

        // Initially there is no list value.
        assert_eq!(form_list_len(&handle, "emails"), 0);
        assert!(form_list_get(&handle, "emails").is_empty());

        // Insert two items.
        form_list_insert(&handle, "emails", 0, Value::String("a@example.com".into()));
        form_list_insert(&handle, "emails", 1, Value::String("b@example.com".into()));

        let items = form_list_get(&handle, "emails");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], Value::String("a@example.com".into()));
        assert_eq!(items[1], Value::String("b@example.com".into()));

        // Remove first item.
        form_list_remove(&handle, "emails", 0);
        let items_after_remove = form_list_get(&handle, "emails");
        assert_eq!(items_after_remove.len(), 1);
        assert_eq!(items_after_remove[0], Value::String("b@example.com".into()));
    }

    #[test]
    fn validate_all_fails_for_required_when_values_empty() {
        let handle = FormHandle::new();
        let registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        registry.borrow_mut().insert(
            "username".to_string(),
            vec![FormRule {
                required: true,
                message: Some("请输入用户名".into()),
                ..FormRule::default()
            }],
        );

        // 初始状态：没有任何表单值，但存在 required 规则，应当校验失败。
        let ok = validate_all(&handle, &registry);
        assert!(!ok);
        let errors = handle.errors();
        assert_eq!(errors.get("username"), Some(&"请输入用户名".to_string()));
    }

    // This scenario depends on the Dioxus runtime because `set_field_value` and
    // `reset_fields` will schedule view updates. In the library tests we don't
    // bootstrap a full runtime, so we keep this test ignored to avoid panics.
    #[test]
    #[ignore]
    fn validate_all_fails_again_after_reset() {
        let handle = FormHandle::new();
        let registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        registry.borrow_mut().insert(
            "username".to_string(),
            vec![FormRule {
                required: true,
                message: Some("请输入用户名".into()),
                ..FormRule::default()
            }],
        );

        // 填写并第一次提交：应当通过。
        handle.set_field_value("username", Value::String("alice".into()));
        let ok_first = validate_all(&handle, &registry);
        assert!(ok_first);

        // 重置后再次提交：应当失败。
        handle.reset_fields();
        let ok_after_reset = validate_all(&handle, &registry);
        assert!(!ok_after_reset);
        let errors_after_reset = handle.errors();
        assert_eq!(
            errors_after_reset.get("username"),
            Some(&"请输入用户名".to_string())
        );
    }
}
