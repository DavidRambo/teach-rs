use clap::{Args, Parser, Subcommand};
use quizzer::add::add;
use quizzer::interface::Interface;
use quizzer::quiz::run_quiz;
use quizzer::real_interface::RealInterface;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds questions to the quiz
    // Add(AddArgs),
    Add,

    /// Runs the quiz
    Quiz,
}

#[derive(Args)]
struct AddArgs {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

fn run(cli: Cli, inter: &mut dyn Interface) {
    match &cli.command {
        Some(Commands::Add) => {
            add(inter);
        }
        Some(Commands::Quiz) => run_quiz(inter),
        &None => inter.write_stdout_line("Try ./quizzer --help"),
    }
}

fn main() {
    let cli = Cli::parse();
    let mut inter = RealInterface::new();

    run(cli, &mut inter);
}

#[cfg(test)]
mod tests {
    use super::*;

    use quizzer::fake_interface::FakeInterface;

    #[test]
    fn add_asks_for_question() {
        let cli = Cli {
            command: Some(Commands::Add),
        };
        let mut inter = FakeInterface::new();

        run(cli, &mut inter);

        assert_eq!(inter.stdout(), "Enter a new question:\n");
    }

    #[test]
    fn quiz_says_no_questions() {
        let cli = Cli {
            command: Some(Commands::Quiz),
        };
        let mut inter = FakeInterface::new();

        run(cli, &mut inter);

        assert_eq!(
            inter.stdout(),
            "There are no quiz questions.\nFirst add some with ./quizzer add\n"
        );
    }

    #[test]
    fn quiz_asks_question() {
        let cli = Cli {
            command: Some(Commands::Quiz),
        };
        let mut inter = FakeInterface::new();

        run(cli, &mut inter);

        assert_eq!(inter.stdout(), "What's my name?\n");
    }
}
