use crate::Status;

pub(crate) trait VmHost {
    fn report_status(&self, _status: Status) {}
}
