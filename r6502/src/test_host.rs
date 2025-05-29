use crate::VMHost;
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

impl VMHost for TestHost {
    fn write_stdout(&self, c: char) {
        self.stdout.borrow_mut().push(c)
    }
}
