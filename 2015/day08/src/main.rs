use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use unescape::unescape;

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut total_orig = 0;
    let mut total_escaped = 0;
    let mut total_encoded = 0;

    for row in reader.lines() {
        let (len, escaped, encoded) = str_lengths(row.unwrap().as_str());

        total_orig += len;
        total_escaped += escaped;

        total_encoded += encoded;
    }

    println!("P1> Number: {}", total_orig - total_escaped);
    println!("P2> Number: {}", total_encoded - total_orig);

    Ok(())
}

fn str_lengths(input: &str) -> (usize, usize, usize) {
    let literal = String::from(input.trim());

    let escaped = unescape(literal.as_str()).unwrap();

    let encoded_length = literal.chars().map(|c| match c {
       '\\'  => 2,
       '\"' => 2,
       _ => 1
    }).sum::<usize>();

    (literal.chars().count(), escaped.chars().count()-2, encoded_length + 2)// The +/- 22 is from the wrapping quotes
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
        let (r1, r2, _) = str_lengths("\"\"");
        assert_eq!(r1, 2);
        assert_eq!(r2, 0);
    }

    #[test]
    fn case_2() {
        let (r1, r2, _) = str_lengths("\"abc\"");
        assert_eq!(r1, 5);
        assert_eq!(r2, 3);
    }

    #[test]
    fn case_3() {
        let (r1, r2, _) = str_lengths("\"aaa\\\"aaa\"");
        assert_eq!(r1, 10);
        assert_eq!(r2, 7);
    }

    #[test]
    fn case_4() {
        let (r1, r2, _) = str_lengths("\"\\x27\"");
        assert_eq!(r1, 6);
        assert_eq!(r2, 1);
    }

    //
    // Part 2
    //
    #[test]
    fn case_5() {
        let (_, _, r) = str_lengths("\"\"");
        assert_eq!(r, 6);
    }

    #[test]
    fn case_6() {
        let (_, _, r) = str_lengths("\"abc\"");
        assert_eq!(r, 9);
    }

    #[test]
    fn case_7() {
        let (_, _, r) = str_lengths("\"aaa\\\"aaa\"");
        assert_eq!(r, 16);
    }

    #[test]
    fn case_8() {
        let (_, _, r) = str_lengths("\"\\x27\"");
        assert_eq!(r, 11);
    }
}
