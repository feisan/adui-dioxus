use adui_dioxus::ThemeProvider;
use adui_dioxus::components::overlay::{OverlayKind, use_overlay_provider};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            OverlayDemo {}
        }
    }
}

#[component]
fn OverlayDemo() -> Element {
    // Install an overlay provider for this demo tree.
    let handle = use_overlay_provider();
    let last_key = use_signal(|| None::<u64>);

    let snapshot = handle.snapshot();
    let overlays: Vec<_> = snapshot
        .entries()
        .map(|(key, meta)| (key.as_u64(), *meta))
        .collect();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base); color: var(--adui-color-text);",
            h2 { "Overlay demo" }
            p { "This internal demo exercises the OverlayManager API (open / update / close / close_all)." }

            div {
                style: "display: flex; gap: 8px; margin-bottom: 16px;",
                {
                    let handle_open = handle.clone();
                    let mut last_key_open = last_key;
                    rsx!(button {
                        onclick: move |_| {
                            // For the demo we treat all overlays as generic modals with mask.
                            let (key, _meta) = handle_open.open(OverlayKind::Modal, true);
                            last_key_open.set(Some(key.as_u64()));
                        },
                        "Open overlay",
                    })
                }
                {
                    let handle_close_last = handle.clone();
                    let last_key_read = last_key;
                    rsx!(button {
                        onclick: move |_| {
                            if let Some(id) = *last_key_read.read() {
                                // Close the last opened overlay if it is still present.
                                let snapshot = handle_close_last.snapshot();
                                for (key, _meta) in snapshot.entries() {
                                    if key.as_u64() == id {
                                        handle_close_last.close(*key);
                                        break;
                                    }
                                }
                            }
                        },
                        "Close last",
                    })
                }
                {
                    let handle_close_all = handle.clone();
                    let mut last_key_clear = last_key;
                    rsx!(button {
                        onclick: move |_| {
                            handle_close_all.close_all();
                            last_key_clear.set(None);
                        },
                        "Close all",
                    })
                }
            }

            p { "Active overlays: {overlays.len()}" }

            // Render simple visual blocks for each active overlay to verify z-index stacking.
            // In a real App, Modal / Drawer / Message / Notification will each render more
            // sophisticated UI based on their own state, but all share z-index coordination.
            {overlays.iter().enumerate().map(|(idx, (id, meta))| {
                let label = format!("Overlay #{idx} (id={id}, z-index={})", meta.z_index);
                let style = format!("position: fixed; inset: 0; background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: center; z-index: {}", meta.z_index);
                rsx! {
                    div {
                        key: "overlay-{id}",
                        style: style,
                        div {
                            style: "padding: 16px 24px; background: white; border-radius: 4px; box-shadow: 0 8px 24px rgba(0,0,0,0.2);",
                            "{label}"
                        }
                    }
                }
            })}
        }
    }
}
