use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::shared_map_core::SharedMapCore;
use crate::shared_map_handle::SharedMapHandle;

/// A thread-private facade to a shared map.
pub struct SharedMap<K, V> {
    core: Arc<SharedMapCore<K, V>>,

    _make_this_struct_not_send_and_not_sync: PhantomData<*mut ()>
}

impl<K, V> SharedMap<K, V>
where
    K: Eq + Hash,
{
    /// Creates a thread-private facade to a shared map.
    pub fn new(handle: &SharedMapHandle<K, V>) -> Self {
        Self {
            core: handle.core(),
            _make_this_struct_not_send_and_not_sync: PhantomData,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // the SharedMap type is currently a do-nothing pass-through. If it actually maintained
        // some state, this is where it would be manipulated, before synchronizing as needed with
        // the core object.
        self.core.insert(key, value)
    }
}
