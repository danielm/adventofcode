use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use unescape::unescape;

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut total_orig = 0;
    let mut total_escaped = 0;

    for row in reader.lines() {
        let (len, escaped) = str_lengths(row.unwrap().as_str());

        total_orig += len;
        total_escaped += escaped;
    }

    println!("P1> Number: {}", total_orig - total_escaped);

    Ok(())
}

fn str_lengths(input: &str) -> (usize, usize) {
    let literal = String::from(input.trim());

    let escaped = unescape(literal.as_str()).unwrap();

    (literal.chars().count(), escaped.chars().count()-2)// The -2 is from the wrapping quotes
}

///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use super::*;

    //
    // Part 1 
    //
    #[test]
    fn case_1() {
        let (r1, r2) = str_lengths("\"\"");
        assert_eq!(r1, 2);
        assert_eq!(r2, 0);
    }

    #[test]
    fn case_2() {
        let (r1, r2) = str_lengths("\"abc\"");
        assert_eq!(r1, 5);
        assert_eq!(r2, 3);
    }

    #[test]
    fn case_3() {
        let (r1, r2) = str_lengths("\"aaa\\\"aaa\"");
        assert_eq!(r1, 10);
        assert_eq!(r2, 7);
    }

    #[test]
    fn case_4() {
        let (r1, r2) = str_lengths("\"\\x27\"");
        assert_eq!(r1, 6);
        assert_eq!(r2, 1);
    }
}
