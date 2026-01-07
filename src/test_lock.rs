use std::sync::{Mutex, Once};

pub static TEST_LOCK: Mutex<()> = Mutex::new(());
static INIT: Once = Once::new();

pub fn lock() -> std::sync::MutexGuard<'static, ()> {
    INIT.call_once(|| {
        crate::initialize().expect("initialize for tests");
    });
    TEST_LOCK.lock().unwrap()
}
