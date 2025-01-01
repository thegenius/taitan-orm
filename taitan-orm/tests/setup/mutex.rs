use std::sync::{Arc, Mutex, OnceLock};

static GLOBAL_TEST_MUTEX: OnceLock<Arc<Mutex<()>>> = OnceLock::new();

pub fn get_test_mutex() -> &'static Arc<Mutex<()>> {
    GLOBAL_TEST_MUTEX.get_or_init(|| Arc::new(Mutex::new(())))
}
