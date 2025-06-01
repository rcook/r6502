use crate::VmHost;
use std::cell::RefCell;

pub(crate) struct TestHost {
    stdout: RefCell<String>,
}

#[allow(unused)]
impl TestHost {
    pub(crate) fn new() -> Self {
        Self {
            stdout: RefCell::new(String::new()),
        }
    }

    pub(crate) fn stdout(self) -> String {
        self.stdout.take()
    }
}

impl VmHost for TestHost {}
