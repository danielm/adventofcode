use std::io::{self};

fn main() -> io::Result<()> {
    let mut result = hash("1321131112", 40);
    println!("P1> Length: {}", result.len());

    result = hash(result.as_str(), 10);// just 10 more iterations
    println!("P2> Length: {}", result.len());

    Ok(())
}

fn hash(input: &str, iterations: i32) -> String {
    let mut result = String::from(input);

    for _ in 0..iterations {
        result = calc(result.as_str());
    }

    result
}

fn calc(input: &str) -> String {
    let mut hash: Vec<String> = Vec::new();

    let tmp = String::from(input);
    let mut chars = tmp.chars().peekable();
    let mut counter = 1;

    while let Some(c) = chars.next() {
        let next = chars.peek();

        if next.is_none() {
            hash.push(format!("{}{}", counter, c));

            break;
        }

        if c == *next.unwrap() {
            counter += 1;
            continue;
        } else {
            hash.push(format!("{}{}", counter, c));
            counter = 1;
        }
    }

    hash.join("")
}

///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = hash("1", 1);

        assert_eq!(result, "11");

    }

    #[test]
    fn case_2() {
        let result = hash("11", 1);

        assert_eq!(result, "21");

    }

    #[test]
    fn case_3() {
        let result = hash("21", 1);

        assert_eq!(result, "1211");

    }

    #[test]
    fn case_4() {
        let result = hash("1211", 1);

        assert_eq!(result, "111221");

    }

    #[test]
    fn case_5() {
        let result = hash("111221", 1);

        assert_eq!(result, "312211");

    }
}
