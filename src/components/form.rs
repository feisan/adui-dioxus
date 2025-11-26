use crate::components::grid::ColProps;
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
    registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
}

#[derive(Clone)]
pub struct FormItemControlContext {
    name: String,
    handle: FormHandle,
    disabled: bool,
    registry: Rc<RefCell<HashMap<String, Vec<FormRule>>>>,
}

impl FormItemControlContext {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> Option<Value> {
        self.handle.get_field_value(&self.name)
    }

    pub fn set_value(&self, value: Value) {
        if self.disabled {
            return;
        }
        self.handle.set_field_value(&self.name, value);
        self.run_validation();
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

pub fn use_form_item_control() -> Option<FormItemControlContext> {
    try_use_context::<FormItemControlContext>()
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
    #[props(optional)]
    pub initial_values: Option<FormValues>,
    #[props(optional)]
    pub form: Option<FormHandle>,
    #[props(optional)]
    pub on_finish: Option<EventHandler<FormFinishEvent>>,
    #[props(optional)]
    pub on_finish_failed: Option<EventHandler<FormFinishFailedEvent>>,
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
        initial_values,
        form,
        on_finish,
        on_finish_failed,
        children,
    } = props;

    let handle = form.unwrap_or_else(FormHandle::new);
    if let Some(initial) = initial_values {
        if handle.values().is_empty() {
            for (k, v) in initial.into_iter() {
                handle.set_field_value(&k, v);
            }
        }
    }

    let registry = Rc::new(RefCell::new(HashMap::new()));
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
        registry: registry.clone(),
    };
    use_context_provider(|| context);

    let submit_handle = handle.clone();
    let submit_registry = registry.clone();
    let finish_cb = on_finish.clone();
    let failed_cb = on_finish_failed.clone();

    rsx! {
        form {
            class: form_class(layout, size),
            onsubmit: move |evt| {
                evt.prevent_default();
                if validate_all(&submit_handle, &submit_registry) {
                    if let Some(cb) = finish_cb.as_ref() {
                        cb.call(FormFinishEvent { values: submit_handle.values() });
                    }
                } else if let Some(cb) = failed_cb.as_ref() {
                    cb.call(FormFinishFailedEvent { errors: submit_handle.errors() });
                }
            },
            {children}
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
    let names = registry
        .try_borrow()
        .map(|map| map.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    let mut ok = true;
    for name in names {
        if !apply_validation_result(handle, registry, &name) {
            ok = false;
        }
    }
    ok
}

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
        class,
        style,
        has_feedback,
        children,
    } = props;

    let item_rules = rules.unwrap_or_default();
    let is_required = item_rules.iter().any(|rule| rule.required);

    if let Some(field_name) = name.clone() {
        {
            ctx.registry
                .borrow_mut()
                .insert(field_name.clone(), item_rules.clone());
        }
        let item_scope = current_scope_id();
        ctx.handle.register_listener(&field_name, item_scope);
        let control_ctx = FormItemControlContext {
            name: field_name.clone(),
            handle: ctx.handle.clone(),
            disabled: ctx.disabled,
            registry: ctx.registry.clone(),
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

    if let Some(len_target) = rule.len {
        if value_length(value) != Some(len_target) {
            return Some(
                rule.message
                    .clone()
                    .unwrap_or_else(|| format!("长度必须为 {}", len_target)),
            );
        }
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

    if let Some(pattern) = &rule.pattern {
        if let Some(text) = value_to_string(value) {
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
    }

    if let Some(validator) = rule.validator {
        if let Err(err) = validator(value) {
            return Some(err);
        }
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
