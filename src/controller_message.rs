pub(crate) enum ControllerMessage {
    WriteStdout(char),
    Println(String),
    Step,
}
