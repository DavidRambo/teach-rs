use std::fmt;

// From https://chrismorgan.info/blog/rust-fizzbuzz/
pub enum Term {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u32),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Term::Fizz => f.write_str("Fizz"),
            Term::Buzz => f.write_str("Buzz"),
            Term::FizzBuzz => f.write_str("FizzBuzz"),
            Term::Number(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub fn fizz_buzz(i: u32) -> Term {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => Term::FizzBuzz,
        (true, false) => Term::Fizz,
        (false, true) => Term::Buzz,
        (false, false) => Term::Number(i),
    }
}

// use std::borrow::Cow;

/// Very naive implementation of FizzBuzz
// Idea: instead of outputting a String, pass in a reference to something that
// can hold the result. Perhaps treat it as bytes? But does the length need to be known?
// pub fn fizz_buzz(i: u32) -> Cow<'static, str> {
//     match (i % 3 == 0, i % 5 == 0) {
//         (true, true) => "FizzBuzz".into(),
//         (true, false) => "Fizz".into(),
//         (false, true) => "Buzz".into(),
//         (false, false) => i.to_string().into(),
//     }
// }

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_file() {
        let answers = include_str!("../fizzbuzz.out").lines();
        for (ln, ans) in answers.enumerate() {
            assert_eq!(fizz_buzz(1 + ln as u32).to_string(), ans);
        }
    }
}
