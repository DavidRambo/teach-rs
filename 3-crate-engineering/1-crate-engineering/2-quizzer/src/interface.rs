pub trait Interface {
    fn read_stdin_line(&mut self) -> Option<String>;
    fn write_stdout(&mut self, text: &str);
    fn write_stdout_line(&mut self, text: &str);
    fn read_quiz_json(&mut self) -> anyhow::Result<String>;
    fn write_quiz_json(&mut self, text: &str) -> anyhow::Result<()>;
}
