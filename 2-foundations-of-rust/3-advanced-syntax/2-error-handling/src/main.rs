/// While taking a first stab at programs, using panic!() is a quick-and-dirty way to do error handling; but panic!() has the obvious drawback
/// that it is all-or-nothing: you cannot recover from it (in general).
// Consider this "interactive hello world" (that is a bit fussy about what is a valid name), where the intent is that the program repeats
// the question if the user entered an invalid name.
//
// 1) Take a few moments to read and understand the various parts of this program.
// Note: we are sorry about the poor formatting, please fix it yourself ('cargo fmt' is your friend, note the changes it makes!)
//
// 2) Besides the explicit panic!()'s, there is a second source of errors that this program currently ignores; what are these?
//
// 3) We have provided an error type for properly reporting all errors that get_username() might generate; change the function get_username
// so that it returns a Result<String, MyError>.
//
// Tip: to keep your main() working while making this change, you can temporarily replace get_username() with get_username().unwrap()
// (this will still result in a panic! in case get_username() returns an error)
//
// 4) Finally, handle the errors in main() properly: an IOError should quit the program, but after an InvalidName error it should repeat
// the question to the user.
//
// NOTE: You will (hopefully) discover that "?" doesn't work in this context, and the resulting code
// is a bit explicit about the errors --- we can solve that with traits, next week!
use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum MyError {
    InvalidName,
    IOError(io::Error),
}

impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        MyError::IOError(value)
    }
}

fn get_username() -> Result<String, MyError> {
    print!("Username: ");
    io::stdout().flush()?; // '?' converts to MyError::IOError b/c of impl From.

    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    input = input.trim().to_string();

    for c in input.chars() {
        if !char::is_alphabetic(c) {
            return Err(MyError::InvalidName);
        }
    }

    if input.is_empty() {
        return Err(MyError::InvalidName);
    }

    Ok(input)
}

fn main() {
    loop {
        let name = get_username();
        match name {
            Ok(name) => {
                println!("Hello {name}!");
                break;
            }
            Err(MyError::IOError(err)) => {
                eprintln!("IOError: {err}");
                break;
            }
            Err(MyError::InvalidName) => println!("Invalid name for this program."),
        }
    }
}
