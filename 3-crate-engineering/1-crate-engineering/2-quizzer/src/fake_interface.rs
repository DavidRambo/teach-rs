use crate::interface::Interface;

pub struct FakeInterface {
    // NOTE: alternatively, hold a &'static str and an iterator over it.
    // Or use <'a> lifetime parameter with the FakeIterface and the &'a str
    stdin: String,
    std_lines_read: usize,
    stdout: String,
    saved_json: Option<String>,
}

impl FakeInterface {
    pub fn new() -> Self {
        Self::with_stdin("")
    }

    /// Creates a FakeInterface with the provided text set as the stdin.
    pub fn with_stdin(text: &str) -> Self {
        Self {
            stdin: text.to_string(),
            std_lines_read: 0,
            stdout: String::new(),
            saved_json: None,
        }
    }

    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    pub fn quiz_file(&self) -> String {
        self.saved_json
            .clone()
            .expect("FakeInterface.quiz_file: No json was saved.")
    }
}

impl Interface for FakeInterface {
    fn read_stdin_line(&mut self) -> Option<String> {
        let next = self.stdin.lines().skip(self.std_lines_read).next();
        match next {
            Some(s) => {
                self.std_lines_read += 1;
                Some(s.to_string())
            }
            None => {
                panic!("This should never happen.")
            }
        }
    }

    fn write_stdout(&mut self, text: &str) {
        self.stdout.push_str(text);
    }

    fn write_stdout_line(&mut self, text: &str) {
        self.stdout.push_str(text);
        self.stdout.push('\n');
    }

    fn read_quiz_json(&mut self) -> anyhow::Result<String> {
        match &self.saved_json {
            Some(contents) => Ok(contents.clone()),
            None => Ok(String::new()),
        }
    }

    /// Writes quiz contents to the quiz file.
    /// Relies on [add] to update the existing quiz.
    fn write_quiz_json(&mut self, text: &str) -> anyhow::Result<()> {
        self.saved_json = Some(text.to_string());
        Ok(())
    }
}
