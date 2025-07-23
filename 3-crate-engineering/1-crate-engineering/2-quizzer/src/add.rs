use crate::{
    interface::Interface,
    quiz::{Question, load_quiz, save_quiz},
};

pub fn add(inter: &mut dyn Interface) {
    let res = load_quiz(inter);
    let mut quiz = match res {
        Ok(q) => q,
        Err(e) => {
            inter.write_stdout_line(&format!("Failed to load quiz.json: {}", e));
            return;
        }
    };

    loop {
        inter.write_stdout_line("Enter a new question:");
        let Some(q_text) = inter.read_stdin_line() else {
            return;
        };

        inter.write_stdout_line("Enter the correct answer:");
        let Some(correct) = inter.read_stdin_line() else {
            return;
        };

        let mut incorrects: Vec<String> = Vec::with_capacity(3);
        let ordinals = &["first", "second", "third"];

        for i in 0..3 {
            inter.write_stdout_line(&format!("Enter the {} incorrect answer:", ordinals[i]));
            if let Some(ans) = inter.read_stdin_line() {
                incorrects.push(ans);
            } else {
                return;
            };
        }

        let new_question = Question {
            text: q_text,
            correct: correct,
            incorrects: incorrects,
        };

        quiz.questions.push(new_question);

        inter.write_stdout_line("Question added!");

        inter.write_stdout_line("Continue adding questions? (y/n)");
        if let Some(ans) = inter.read_stdin_line() {
            if ans.trim().to_lowercase() != "y" {
                let res = save_quiz(inter, &quiz);
                if let Err(e) = res {
                    inter.write_stdout_line(&format!("Failed to save quiz to 'quiz.json': {}", e));
                }

                break;
            } else {
                continue;
            }
        } else {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::fake_interface::FakeInterface;

    use super::*;

    #[test]
    fn add_gets_question_and_answers() {
        let mut inter = FakeInterface::with_stdin(
            "\
            What's my name?\n\
            quizzer\n\
            andy\n\
            sam\n\
            joe\n\
            n\n\
        ",
        );

        add(&mut inter);

        assert_eq!(
            inter.stdout(),
            "\
            Enter a new question:\n\
            Enter the correct answer:\n\
            Enter the first incorrect answer:\n\
            Enter the second incorrect answer:\n\
            Enter the third incorrect answer:\n\
            Question added!\n\
            Continue adding questions? (y/n)\n\
            "
        );
    }

    #[test]
    fn two_questions_stored_in_json() {
        let mut inter = FakeInterface::with_stdin(
            "\
            What's my name?\n\
            quizzer\n\
            andy\n\
            sam\n\
            joe\n\
            y\n\
            What is 2 + 2?\n\
            4\n\
            3\n\
            5\n\
            0\n\
            n\n\
        ",
        );

        add(&mut inter);

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&inter.quiz_file())
                .expect("Failed to deserialize quiz file"),
            json!({
                "questions": [
                    {
                        "text": "What's my name?",
                        "correct": "quizzer",
                        "incorrects": ["andy", "sam", "joe"],
                    },
                    {
                        "text": "What is 2 + 2?",
                        "correct": "4",
                        "incorrects": ["3", "5", "0"],
                    }
                ]
            })
        );
    }
}
