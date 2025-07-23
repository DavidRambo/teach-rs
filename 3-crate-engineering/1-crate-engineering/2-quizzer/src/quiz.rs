use rand::Rng;

use serde::{Deserialize, Serialize};

use crate::interface::Interface;

#[derive(Default, Serialize, Deserialize)]
pub struct Quiz {
    pub questions: Vec<Question>,
}

impl Quiz {
    /// Returns the number of questions in the quiz.
    pub fn len(&self) -> usize {
        self.questions.len()
    }

    /// Adds new question(s) to the quiz's questions bank.
    pub fn append_questions(&mut self, new_questions: Vec<Question>) -> anyhow::Result<()> {
        self.questions.extend(new_questions);
        Ok(())
    }

    /// Returns true if the quiz has no questions.
    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub correct: String,
    pub incorrects: Vec<String>,
}

pub fn run_quiz(inter: &mut dyn Interface) {
    let load_res = load_quiz(inter);
    let quiz = match load_res {
        Ok(q) => q,
        Err(e) => {
            inter.write_stdout_line(&format!("Failed to load quiz.json: {}", e));
            return;
        }
    };

    if quiz.is_empty() {
        inter.write_stdout_line("There are no quiz questions.\nFirst add some with ./quizzer add");
        return;
    }

    let mut score: u32 = 0;

    for q in quiz.questions.iter() {
        inter.write_stdout(&q.text);

        inter.write_stdout_line(&fmt_ans_choices(&q.correct, &q.incorrects));
        inter.write_stdout(">>> ");

        let ans = inter.read_stdin_line().unwrap();

        if ans == q.correct {
            score += 1;
            inter.write_stdout_line("Correct!\n");
        } else {
            inter.write_stdout_line("Wrong answer.\n");
        }
    }

    inter.write_stdout_line(&format!("You scored {score} out of {} points.", quiz.len()));
}

/// Displays the answers choices in a random order.
fn fmt_ans_choices(correct: &str, incorrects: &Vec<String>) -> String {
    let mut output = String::new();

    let show_correct = rand::rng().random_range(0..3);

    for (idx, ans) in incorrects.iter().enumerate() {
        if idx == show_correct {
            output.push_str(&format!("\n -> {}", correct));
        }
        output.push_str("\n -> ");
        output.push_str(ans);
    }

    output
}

/// Loads the deserialized quiz if there is one, otherwise returns an empty quiz.
pub fn load_quiz(inter: &mut dyn Interface) -> anyhow::Result<Quiz> {
    let res = inter.read_quiz_json();
    match res {
        Ok(contents) => {
            if contents.is_empty() {
                Ok(Quiz::default())
            } else {
                let deser_res = serde_json::from_str(&contents);
                match deser_res {
                    Ok(r) => Ok(r),
                    Err(r) => Err(anyhow::anyhow!(r)),
                }
            }
        }
        Err(e) => Err(e),
    }
}

pub fn save_quiz(inter: &mut dyn Interface, questions: &Quiz) -> anyhow::Result<()> {
    let json = serde_json::to_string(questions)?;
    inter.write_quiz_json(&json)?;
    Ok(())
}
