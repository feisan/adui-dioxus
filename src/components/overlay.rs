use dioxus::prelude::*;
use std::collections::HashMap;

/// Kinds of overlay layers that share a common z-index space.
///
/// This is intentionally small. If we need more kinds in the future we can
/// extend this enum without breaking existing behaviour.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OverlayKind {
    /// Lightweight dropdown-like overlays used by selector components.
    Dropdown,
    Message,
    Notification,
    Modal,
    Drawer,
}

/// Identifier for a single overlay instance managed by `OverlayManager`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OverlayKey(u64);

impl OverlayKey {
    /// Expose the raw numeric id for logging or debugging purposes.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

/// Lightweight metadata attached to a registered overlay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverlayMeta {
    pub kind: OverlayKind,
    pub z_index: i32,
    pub has_mask: bool,
}

/// Internal state for all overlays within a single App tree.
///
/// The manager does not know about the concrete UI of each overlay type. It
/// only allocates z-index values and tracks basic metadata so that visual
/// layers can be rendered in a stable order.
#[derive(Clone, Debug)]
pub struct OverlayManager {
    next_key: u64,
    base_z_index: i32,
    step: i32,
    entries: HashMap<OverlayKey, OverlayMeta>,
}

impl Default for OverlayManager {
    fn default() -> Self {
        Self {
            next_key: 1,
            // AntD 的浮层默认 z-index 区间大致在 1000+，这里选择一个接近的起点，
            // 后续如有需要可以通过 ConfigProvider 提供全局配置入口。
            base_z_index: 1000,
            // 为不同 overlay 预留间隔，便于未来在类型内部再做细分。
            step: 10,
            entries: HashMap::new(),
        }
    }
}

impl OverlayManager {
    /// Open a new overlay of the given kind.
    ///
    /// Returns the allocated key together with the computed metadata so the
    /// caller can immediately use the z-index.
    pub fn open(&mut self, kind: OverlayKind, has_mask: bool) -> (OverlayKey, OverlayMeta) {
        let key = OverlayKey(self.next_key);
        self.next_key += 1;

        let z_index = self.next_z_index();
        let meta = OverlayMeta {
            kind,
            z_index,
            has_mask,
        };
        self.entries.insert(key, meta);
        (key, meta)
    }

    /// Update a subset of the metadata for an existing overlay.
    pub fn update(&mut self, key: OverlayKey, has_mask: Option<bool>) -> Option<OverlayMeta> {
        if let Some(entry) = self.entries.get_mut(&key) {
            if let Some(mask) = has_mask {
                entry.has_mask = mask;
            }
            return Some(*entry);
        }
        None
    }

    /// Close a single overlay.
    pub fn close(&mut self, key: OverlayKey) {
        self.entries.remove(&key);
    }

    /// Close all overlays managed by this instance.
    pub fn close_all(&mut self) {
        self.entries.clear();
    }

    /// Return an iterator over all active overlays. The order of iteration is
    /// not guaranteed to be stable; callers that care about z-index ordering
    /// should sort by `z_index`.
    pub fn entries(&self) -> impl Iterator<Item = (&OverlayKey, &OverlayMeta)> {
        self.entries.iter()
    }

    /// Return the highest z-index currently allocated, or the base value if
    /// there are no overlays.
    pub fn current_top_z_index(&self) -> i32 {
        self.entries
            .values()
            .map(|m| m.z_index)
            .max()
            .unwrap_or(self.base_z_index)
    }

    fn next_z_index(&self) -> i32 {
        let top = self
            .entries
            .values()
            .map(|m| m.z_index)
            .max()
            .unwrap_or(self.base_z_index - self.step);
        top + self.step
    }
}

/// Handle used by components to interact with the overlay manager through a
/// Dioxus signal. This is designed to be provided once near the root of the
/// app (for example inside the future `App` component) and then consumed via
/// `use_overlay` in child components.
#[derive(Clone)]
pub struct OverlayHandle {
    state: Signal<OverlayManager>,
}

impl OverlayHandle {
    /// Register a new overlay and receive its key and metadata.
    pub fn open(&self, kind: OverlayKind, has_mask: bool) -> (OverlayKey, OverlayMeta) {
        let mut state = self.state;
        state.write().open(kind, has_mask)
    }

    /// Update metadata for an existing overlay.
    pub fn update(&self, key: OverlayKey, has_mask: Option<bool>) -> Option<OverlayMeta> {
        let mut state = self.state;
        state.write().update(key, has_mask)
    }

    /// Close a specific overlay.
    pub fn close(&self, key: OverlayKey) {
        let mut state = self.state;
        state.write().close(key);
    }

    /// Close all overlays.
    pub fn close_all(&self) {
        let mut state = self.state;
        state.write().close_all();
    }

    /// Snapshot the current manager state. This is intended for read-only
    /// operations such as rendering overlay layers.
    pub fn snapshot(&self) -> OverlayManager {
        self.state.read().clone()
    }
}

/// Create an `OverlayHandle` and install it into the current scope's context.
///
/// This should typically be called once in a high-level component that owns
/// the overlay surface (for example the `App` component in a real应用). Child
/// components can then obtain the handle via [`use_overlay`].
pub fn use_overlay_provider() -> OverlayHandle {
    let state = use_signal(OverlayManager::default);
    let handle = OverlayHandle { state };
    use_context_provider(|| handle.clone());
    handle
}

/// Retrieve the `OverlayHandle` from context if it has been installed.
///
/// This returns `None` when no provider is present. Callers may choose to
/// fall back to local behaviour in that case.
pub fn use_overlay() -> Option<OverlayHandle> {
    try_use_context::<OverlayHandle>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_manager_allocates_monotonic_z_indices() {
        let mut mgr = OverlayManager::default();
        let (_k1, m1) = mgr.open(OverlayKind::Modal, true);
        let (_k2, m2) = mgr.open(OverlayKind::Drawer, false);
        assert!(m2.z_index > m1.z_index);
        assert_eq!(mgr.current_top_z_index(), m2.z_index);
    }

    #[test]
    fn overlay_manager_update_and_close_work() {
        let mut mgr = OverlayManager::default();
        let (key, meta) = mgr.open(OverlayKind::Message, false);
        assert!(!meta.has_mask);
        let updated = mgr.update(key, Some(true)).unwrap();
        assert!(updated.has_mask);
        mgr.close(key);
        assert_eq!(mgr.entries().count(), 0);
    }
}
