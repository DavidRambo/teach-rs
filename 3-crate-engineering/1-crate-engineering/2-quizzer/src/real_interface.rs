use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read, Stdin, Stdout, Write, stdin, stdout};

use crate::interface::Interface;

pub struct RealInterface {
    stdin_lines: Lines<BufReader<Stdin>>,
    stdout: Stdout,
}

impl RealInterface {
    pub fn new() -> Self {
        Self {
            // Must bring in BufRead trait in order to call .lines() method.
            stdin_lines: BufReader::new(stdin()).lines(),
            stdout: stdout(),
        }
    }
}

impl Interface for RealInterface {
    fn read_stdin_line(&mut self) -> Option<String> {
        self.stdin_lines
            .next() // get the next Lines BufRead line -> Option<Result<String, std::io::Error>>
            .transpose() // Swap Option and Result -> Result<Option<String>, std::io::Error>
            .expect("Failed to read stdin") // If something went wrong, then panic.
    }

    fn write_stdout(&mut self, text: &str) {
        write!(self.stdout, "{}", text).expect("Failed to write stdout");
        self.stdout.flush().unwrap();
    }

    fn write_stdout_line(&mut self, text: &str) {
        writeln!(self.stdout, "{}", text).expect("Failed to write stdout");
    }

    fn read_quiz_json(&mut self) -> anyhow::Result<String> {
        let mut file = File::open("quiz.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Writes quiz contents to the quiz file.
    /// Relies on [add] to update the existing quiz.
    fn write_quiz_json(&mut self, text: &str) -> anyhow::Result<()> {
        let mut file = File::create("quiz.json")?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }
}
