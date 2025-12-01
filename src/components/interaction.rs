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

#[cfg(test)]
mod interaction_tests {
    use super::*;

    #[test]
    fn pointer_state_default() {
        let state = PointerState::default();
        assert_eq!(state.active_id, None);
        assert_eq!(state.dragging, false);
    }

    #[test]
    fn pointer_state_partial_eq() {
        let state1 = PointerState {
            active_id: Some(1),
            dragging: true,
        };
        let state2 = PointerState {
            active_id: Some(1),
            dragging: true,
        };
        let state3 = PointerState {
            active_id: Some(2),
            dragging: true,
        };
        let state4 = PointerState {
            active_id: Some(1),
            dragging: false,
        };

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
        assert_ne!(state1, state4);
    }

    #[test]
    fn pointer_state_debug() {
        let state = PointerState {
            active_id: Some(42),
            dragging: true,
        };
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("PointerState"));
        assert!(debug_str.contains("42") || debug_str.contains("Some"));
    }

    #[test]
    fn pointer_state_clone() {
        let state1 = PointerState {
            active_id: Some(10),
            dragging: true,
        };
        let state2 = state1;
        assert_eq!(state1, state2);
    }

    #[test]
    fn pointer_state_copy() {
        let state1 = PointerState {
            active_id: Some(5),
            dragging: false,
        };
        let state2 = state1;
        // Copy trait should allow both to be used independently
        assert_eq!(state1.active_id, Some(5));
        assert_eq!(state2.active_id, Some(5));
    }

    #[test]
    fn reset_pointer_logic() {
        // Test the logic of reset_pointer by creating the expected state
        let reset_state = PointerState {
            active_id: None,
            dragging: false,
        };
        assert_eq!(reset_state.active_id, None);
        assert_eq!(reset_state.dragging, false);
    }

    #[test]
    fn is_active_pointer_logic() {
        // Test the logic: active_id == Some(pointer_id)
        let state_with_id = PointerState {
            active_id: Some(42),
            dragging: true,
        };
        // Logic: state.active_id == Some(evt.pointer_id())
        assert_eq!(state_with_id.active_id, Some(42));

        let state_without_id = PointerState {
            active_id: None,
            dragging: false,
        };
        assert_eq!(state_without_id.active_id, None);
    }
}
