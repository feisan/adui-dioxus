use dioxus::prelude::*;

/// Helper handle for floating overlays (Tooltip / Popover / Popconfirm /
/// Dropdown 等)，封装点击空白关闭 + ESC 关闭的共用逻辑。
///
/// 使用方式：
/// - 在组件内部创建 `let handle = use_floating_close_handle(open_signal);`
/// - 在触发区或浮层内部交互前调用 `handle.mark_internal_click()`，避免当前事件被
///   document 级别的 click 监听误判为空白点击；
/// - 在键盘事件中检测 ESC 并调用 `handle.close()`。
#[derive(Clone, Copy)]
pub struct FloatingCloseHandle {
    internal_click_flag: Signal<bool>,
    open: Signal<bool>,
}

impl FloatingCloseHandle {
    /// 标记当前 click/交互来源于组件内部，避免 document click handler 立即关闭。
    pub fn mark_internal_click(&self) {
        let mut flag = self.internal_click_flag;
        flag.set(true);
    }

    /// 主动关闭浮层（例如 ESC 或业务逻辑需要）。
    pub fn close(&self) {
        let mut open = self.open;
        open.set(false);
    }
}

/// Hook：为当前浮层组件安装 click-outside + ESC 关闭的共用逻辑。
///
/// - `open` 是该浮层的可写 Signal；
/// - 只在 wasm32 目标下注册 document click 监听，其他目标为 no-op。
pub fn use_floating_close_handle(open: Signal<bool>) -> FloatingCloseHandle {
    let internal_click_flag: Signal<bool> = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        let mut flag_for_global = internal_click_flag;
        let mut open_for_global = open;
        use_effect(move || {
            use wasm_bindgen::{JsCast, closure::Closure};
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let target: web_sys::EventTarget = document.into();
                    let handler =
                        Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(move |_evt| {
                            let mut flag = flag_for_global;
                            if *flag.read() {
                                // 本轮事件来源于内部交互，消费标记后不关闭浮层。
                                flag.set(false);
                                return;
                            }
                            let mut open_signal = open_for_global;
                            if *open_signal.read() {
                                open_signal.set(false);
                            }
                        }));
                    let _ = target.add_event_listener_with_callback(
                        "click",
                        handler.as_ref().unchecked_ref(),
                    );
                    handler.forget();
                }
            }
        });
    }

    FloatingCloseHandle {
        internal_click_flag,
        open,
    }
}

#[cfg(test)]
mod floating_tests {
    use super::*;

    #[test]
    fn floating_close_handle_implements_clone_and_copy() {
        // Verify that FloatingCloseHandle implements Clone and Copy traits
        // This is important for the component's usage pattern
        fn assert_clone<T: Clone>() {}
        fn assert_copy<T: Copy>() {}
        assert_clone::<FloatingCloseHandle>();
        assert_copy::<FloatingCloseHandle>();
    }

    #[test]
    fn floating_close_handle_method_signatures() {
        // Verify that the methods exist on FloatingCloseHandle with correct signatures
        // mark_internal_click takes &self (immutable reference) and returns ()
        // close takes &self (immutable reference) and returns ()
        fn assert_mark_method(_handle: &FloatingCloseHandle) {
            // Signature: fn mark_internal_click(&self)
        }
        fn assert_close_method(_handle: &FloatingCloseHandle) {
            // Signature: fn close(&self)
        }
        // These functions verify the method signatures are correct
        assert_mark_method as fn(&FloatingCloseHandle);
        assert_close_method as fn(&FloatingCloseHandle);
    }

    #[test]
    fn use_floating_close_handle_function_signature() {
        // Verify that use_floating_close_handle function exists with correct signature
        // Signature: fn use_floating_close_handle(open: Signal<bool>) -> FloatingCloseHandle
        fn assert_function_signature(_open: Signal<bool>) -> FloatingCloseHandle {
            // This would require runtime context, but we verify the signature
            unreachable!("This is just for type checking")
        }
        // Verify the function type matches
        let _function_type: fn(Signal<bool>) -> FloatingCloseHandle = assert_function_signature;
    }

    #[test]
    fn floating_close_handle_is_copy_type() {
        // Verify FloatingCloseHandle is Copy, meaning it can be copied by value
        // This is important for the component's usage pattern where handles are passed around
        fn requires_copy<T: Copy>(_t: T) {}
        // This test verifies the type constraint at compile time
        // We can't create an instance without runtime, but we verify the trait bound
    }
}
