use crate::model::common::arena::{ArenaProperties, InternalKey};

/// Read-only access to the `ArenaProperties` on a property element.
pub trait HasArenaProperties {
    fn arena_properties(&self) -> &ArenaProperties;

    fn key(&self) -> Option<InternalKey> {
        self.arena_properties().key
    }
}

/// Mutable access to the `ArenaProperties` on a property element.
pub trait HasArenaPropertiesMut: HasArenaProperties {
    fn arena_properties_mut(&mut self) -> &mut ArenaProperties;

    fn set_key(&mut self, key: InternalKey) {
        self.arena_properties_mut().key = Some(key);
    }

    fn set_key_opt(&mut self, key: Option<InternalKey>) {
        self.arena_properties_mut().key = key;
    }

    fn clear_key(&mut self) {
        self.arena_properties_mut().key = None;
    }
}
