use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;

/// The implementation logic for a shared map.
///
/// This struct can be accessed concurrently from multiple threads.
pub(crate) struct SharedMapCore<K, V> {
    map: Mutex<HashMap<K, V>>,
}

impl<K, V> SharedMapCore<K, V>
where
    K: Eq + Hash,
{
    pub fn new(capacity: usize) -> SharedMapCore<K, V> {
        Self {
            map: Mutex::new(HashMap::with_capacity(capacity)),
        }
    }

    pub(crate) fn insert(&self, key: K, value: V) {
        let mut m = self.map.lock().unwrap();
        m.insert(key, value);
    }
}
