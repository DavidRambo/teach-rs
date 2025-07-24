use std::borrow::Cow;

/// Very naive implementation of FizzBuzz
// Idea: instead of outputting a String, pass in a reference to something that
// can hold the result. Perhaps treat it as bytes? But does the length need to be known?
pub fn fizz_buzz(i: u32) -> Cow<'static, str> {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => "FizzBuzz".into(),
        (true, false) => "Fizz".into(),
        (false, true) => "Buzz".into(),
        (false, false) => i.to_string().into(),
    }
}

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
            assert_eq!(fizz_buzz(1 + ln as u32), ans);
        }
    }
}
