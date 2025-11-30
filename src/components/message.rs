use crate::components::overlay::{OverlayHandle, OverlayKey, OverlayKind, OverlayMeta};
use dioxus::prelude::*;

/// Message types aligned with Ant Design semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    Info,
    Success,
    Warning,
    Error,
    Loading,
}

/// Configuration for a single message instance.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageConfig {
    pub content: String,
    pub r#type: MessageType,
    /// Auto close delay in seconds. Set to 0 for no auto-dismiss.
    pub duration: f32,
    /// Custom icon element. When None, default icon based on type is used.
    pub icon: Option<Element>,
    /// Additional CSS class.
    pub class: Option<String>,
    /// Inline styles.
    pub style: Option<String>,
    /// Unique key for this message (for programmatic updates).
    pub key: Option<String>,
    /// Callback when message is clicked.
    pub on_click: Option<EventHandler<()>>,
}

impl Default for MessageConfig {
    fn default() -> Self {
        Self {
            content: String::new(),
            r#type: MessageType::Info,
            duration: 3.0,
            icon: None,
            class: None,
            style: None,
            key: None,
            on_click: None,
        }
    }
}

/// Internal representation of an active message.
#[derive(Clone, Debug, PartialEq)]
pub struct MessageEntry {
    pub key: OverlayKey,
    pub meta: OverlayMeta,
    pub config: MessageConfig,
}

/// Signal type used to hold the current message queue.
pub type MessageEntriesSignal = Signal<Vec<MessageEntry>>;

/// Create the message entries signal and install it into context.
///
/// This should be called once near the top of the App tree.
pub fn use_message_entries_provider() -> MessageEntriesSignal {
    let signal: MessageEntriesSignal = use_context_provider(|| Signal::new(Vec::new()));
    signal
}

/// Retrieve the message entries signal from context.
pub fn use_message_entries() -> MessageEntriesSignal {
    use_context::<MessageEntriesSignal>()
}

/// Public API used by `use_message()` callers.
#[derive(Clone)]
pub struct MessageApi {
    overlay: OverlayHandle,
    entries: MessageEntriesSignal,
}

impl MessageApi {
    pub fn new(overlay: OverlayHandle, entries: MessageEntriesSignal) -> Self {
        Self { overlay, entries }
    }

    /// Low-level open method that accepts a full config object.
    pub fn open(&self, config: MessageConfig) -> OverlayKey {
        let (key, meta) = self.overlay.open(OverlayKind::Message, false);
        let mut entries = self.entries;
        entries.write().push(MessageEntry {
            key,
            meta,
            config: config.clone(),
        });

        schedule_message_dismiss(key, self.entries, self.overlay.clone(), config.duration);
        key
    }

    fn open_with_type(&self, content: impl Into<String>, kind: MessageType) -> OverlayKey {
        let cfg = MessageConfig {
            content: content.into(),
            r#type: kind,
            duration: 3.0,
            icon: None,
            class: None,
            style: None,
            key: None,
            on_click: None,
        };
        self.open(cfg)
    }

    pub fn info(&self, content: impl Into<String>) -> OverlayKey {
        self.open_with_type(content, MessageType::Info)
    }

    pub fn success(&self, content: impl Into<String>) -> OverlayKey {
        self.open_with_type(content, MessageType::Success)
    }

    pub fn warning(&self, content: impl Into<String>) -> OverlayKey {
        self.open_with_type(content, MessageType::Warning)
    }

    pub fn error(&self, content: impl Into<String>) -> OverlayKey {
        self.open_with_type(content, MessageType::Error)
    }

    pub fn loading(&self, content: impl Into<String>) -> OverlayKey {
        // Loading 默认不自动关闭，交给调用方控制。
        let cfg = MessageConfig {
            content: content.into(),
            r#type: MessageType::Loading,
            duration: 0.0,
            icon: None,
            class: None,
            style: None,
            key: None,
            on_click: None,
        };
        self.open(cfg)
    }

    /// Destroy a specific message or all messages when `key` is None.
    pub fn destroy(&self, key: Option<OverlayKey>) {
        let mut entries = self.entries;
        match key {
            Some(k) => {
                entries.write().retain(|e| e.key != k);
                self.overlay.close(k);
            }
            None => {
                let current: Vec<_> = entries.read().iter().map(|e| e.key).collect();
                entries.write().clear();
                for k in current {
                    self.overlay.close(k);
                }
            }
        }
    }
}

/// Host component rendering the active message list.
#[component]
pub fn MessageHost() -> Element {
    let entries_signal = use_message_entries();
    let entries = entries_signal.read().clone();

    if entries.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "adui-message-root",
            style: "position: fixed; top: 24px; inset-inline-end: 0; z-index: 1000; display: flex; flex-direction: column; gap: 8px; padding-inline-end: 24px; pointer-events: none;",
            {entries.iter().map(|entry| {
                let key = entry.key;
                let z = entry.meta.z_index;
                let text = entry.config.content.clone();
                let kind_class = match entry.config.r#type {
                    MessageType::Info => "adui-message-info",
                    MessageType::Success => "adui-message-success",
                    MessageType::Warning => "adui-message-warning",
                    MessageType::Error => "adui-message-error",
                    MessageType::Loading => "adui-message-loading",
                };
                rsx! {
                    div {
                        key: "message-{key:?}",
                        class: "adui-message {kind_class}",
                        style: "pointer-events: auto; z-index: {z}; min-width: 200px; max-width: 480px; padding: 8px 16px; border-radius: 4px; background: var(--adui-color-bg-container); box-shadow: var(--adui-shadow); color: var(--adui-color-text); border: 1px solid var(--adui-color-border);",
                        span { "{text}" }
                        button {
                            style: "margin-left: 8px; background: none; border: none; cursor: pointer; color: var(--adui-color-text-secondary);",
                            onclick: move |_| {
                                let mut entries = entries_signal;
                                entries.write().retain(|e| e.key != key);
                            },
                            "×"
                        }
                    }
                }
            })}
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn schedule_message_dismiss(
    key: OverlayKey,
    entries: MessageEntriesSignal,
    overlay: OverlayHandle,
    duration_secs: f32,
) {
    use wasm_bindgen::{JsCast, closure::Closure};

    if duration_secs <= 0.0 {
        return;
    }

    if let Some(window) = web_sys::window() {
        let delay_ms = (duration_secs * 1000.0) as i32;
        let mut entries_signal = entries;
        let overlay_clone = overlay.clone();
        let callback = Closure::once(move || {
            entries_signal.write().retain(|e| e.key != key);
            overlay_clone.close(key);
        });
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            delay_ms,
        );
        callback.forget();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn schedule_message_dismiss(
    _key: OverlayKey,
    _entries: MessageEntriesSignal,
    _overlay: OverlayHandle,
    _duration_secs: f32,
) {
}
