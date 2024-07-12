use std::hash::Hash;
use std::sync::{Arc, OnceLock};
use crate::shared_map_core::SharedMapCore;

/// A handle to a shared map. This can be shared between threads.
#[derive(Clone)]
pub struct SharedMapHandle<K, V> {
    core: OnceLock<Arc<SharedMapCore<K, V>>>,

    // the handle captures the startup arguments since the core object is lazily created
    capacity: usize,
}

impl<K, V> SharedMapHandle<K, V>
where
    K: Eq + Hash,
{
    /// Create a handle to a shared map. Use [`crate::SharedMap::new`] to access the map.
    pub const fn new(capacity: usize) -> Self {
        // In order for this function to be 'const', we can't immediately create the core object
        // here. Instead, we capture the arguments into 'self' and lazily create the core object
        // the first time it is needed in the 'core' function below.

        Self {
            core: OnceLock::new(),
            capacity
        }
    }

    pub(crate) fn core(&self) -> Arc<SharedMapCore<K, V>>
    {
        // lazily create the core object, using the captured arguments in 'self'.
        self.core.get_or_init(|| Arc::new(SharedMapCore::new(self.capacity))).clone()
    }
}
