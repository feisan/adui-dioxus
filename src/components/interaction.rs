//! Shared interaction helpers (pointer capture, dragging state).
//!
//! Centralizes pointer tracking so components like Slider/ColorPicker can reuse the same
//! capture/release semantics and avoid duplicating DOM handling.

use dioxus::events::PointerData;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Simple pointer tracking state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PointerState {
    pub active_id: Option<i32>,
    pub dragging: bool,
}

/// Extract a `PointerEvent` from a Dioxus pointer event payload.
pub fn as_pointer_event(evt: &Event<PointerData>) -> Option<web_sys::PointerEvent> {
    evt.data().downcast::<web_sys::PointerEvent>().cloned()
}

/// Begin tracking a pointer and set capture on the target element.
pub fn start_pointer(state: &mut Signal<PointerState>, evt: &web_sys::PointerEvent) {
    state.set(PointerState {
        active_id: Some(evt.pointer_id()),
        dragging: true,
    });

    if let Some(target) = evt
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        let _ = target.set_pointer_capture(evt.pointer_id());
    }
}

/// Stop tracking when the active pointer finishes, releasing capture if present.
pub fn end_pointer(state: &mut Signal<PointerState>, evt: &web_sys::PointerEvent) {
    let should_end = {
        let reader = state.read();
        reader.active_id == Some(evt.pointer_id())
    };
    if !should_end {
        return;
    }

    if let Some(target) = evt
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
        let _ = target.release_pointer_capture(evt.pointer_id());
    }

    state.set(PointerState {
        active_id: None,
        dragging: false,
    });
}

/// Whether the event belongs to the currently tracked pointer.
pub fn is_active_pointer(state: &Signal<PointerState>, evt: &web_sys::PointerEvent) -> bool {
    state.read().active_id == Some(evt.pointer_id())
}

/// Reset state manually (e.g., when unmounting).
pub fn reset_pointer(state: &mut Signal<PointerState>) {
    state.set(PointerState {
        active_id: None,
        dragging: false,
    });
}
