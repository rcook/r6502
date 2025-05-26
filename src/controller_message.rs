pub(crate) enum ControllerMessage {
    AppendStdoutChar(char),
    AppendLogLine(String),
}
