use crate::shared_map::SharedMap;
use crate::shared_map_handle::SharedMapHandle;

static MAP_HANDLE: SharedMapHandle<String, i32> = SharedMapHandle::new(123);

pub fn use_a_statically_declared_shared_map() {
    let mut m = SharedMap::new(&MAP_HANDLE);
    m.insert("One".to_string(), 1);
}

pub fn use_a_dynamically_declared_shared_map() {
    let h = SharedMapHandle::<String, i32>::new(123);

    // handle can be shared to other threads
    let shared_h = h.clone();
    std::thread::spawn(move || {
        let mut m = SharedMap::new(&shared_h);
        m.insert("Two".to_string(), 2);
    }).join().unwrap();

    let mut m = SharedMap::new(&h);
    m.insert("One".to_string(), 1);
}

pub struct StateInstantiatedPerThread {
    _map: SharedMap<String, i32>,
}

impl StateInstantiatedPerThread {
    pub fn new() -> Self {
        Self {
            _map: SharedMap::new(&MAP_HANDLE),  // grab a thread-private facade
        }
    }
}