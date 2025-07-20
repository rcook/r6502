use std::sync::OnceLock;
use std::thread::{ThreadId, current};

static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

pub fn set_main_thread() {
    MAIN_THREAD_ID.set(current().id()).unwrap();
}

pub fn assert_is_main_thread() {
    assert!(
        !(*MAIN_THREAD_ID.get().unwrap() != current().id()),
        "code must run on main thread"
    );
}
