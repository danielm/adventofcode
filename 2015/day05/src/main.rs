use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut nice_counter: u32 = 0;
    let mut really_nice_counter: u32 = 0;

    for row in reader.lines() {

        let line = row.unwrap();

        if is_nice(&line) {
            nice_counter += 1;
        }

        if is_really_nice(line.as_str()) {
            really_nice_counter += 1;
        }
    }

    println!("P1 > Nice line counter: {}", nice_counter);
    println!("P2 > Really Nice line counter: {}", really_nice_counter);
    
    Ok(())
}

fn is_nice(line: &String) -> bool {
   
    let banned = ["ab", "cd", "pq", "xy"];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for word in banned {
        if line.contains(word) {
            return false;
        }
    }

    let mut last_char = '\0';//Unicode's Null, empy char does not exist
    let mut got_double = false;
    let mut vowel_count = 0;

    for c in line.chars() {
        if c == last_char {
            got_double = true;
        }

        if vowels.contains(&c) {
            vowel_count += 1;
        }

        last_char = c;
    }


    return got_double && vowel_count >= 3;
}

fn is_really_nice(line: &str) -> bool {

    let mut got_pair = false;
    let mut got_jump = false;

    for (i,c) in line.chars().enumerate() {
        if i + 1 >= line.len() {
            break;
        }

        // Look for pairs
        let pair = format!("{}{}", c, &line[i+1..i+2]);

        if line.matches(&pair).count() > 1 {
            got_pair = true;
        }

        // Now for matching bunny jump
        if i + 2 >= line.len() {
            break;
        }

        if c == line[i+2..i+3].chars().nth(0).unwrap() {
            got_jump = true;
        }
    }
    return got_pair && got_jump;
}

///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use crate::{is_nice, is_really_nice};

    //
    // Part 1 
    //
    #[test]
    fn case_ugknbfddgicrmopn() {
        let input = "ugknbfddgicrmopn";

        let result = is_nice(&input.to_string());

        assert_eq!(result, true);
    }

    #[test]
    fn case_aaa() {
        let input = "aaa";

        let result = is_nice(&input.to_string());

        assert_eq!(result, true);
    }

    #[test]
    fn case_jchzalrnumimnmhp() {
        let input = "jchzalrnumimnmhp";

        let result = is_nice(&input.to_string());

        assert_eq!(result, false);
    }

    #[test]
    fn case_haegwjzuvuyypxyu() {
        let input = "haegwjzuvuyypxyu";

        let result = is_nice(&input.to_string());

        assert_eq!(result, false);
    }

    #[test]
    fn case_dvszwmarrgswjxmb() {
        let input = "dvszwmarrgswjxmb";

        let result = is_nice(&input.to_string());

        assert_eq!(result, false);
    }

    //
    // Part 2
    //
    #[test]
    fn case_qjhvhtzxzqqjkmpb() {
        let input = "qjhvhtzxzqqjkmpb";

        let result = is_really_nice(input);

        assert_eq!(result, true);
    }

    #[test]
    fn case_xxyxx() {
        let input = "xxyxx";

        let result = is_really_nice(input);

        assert_eq!(result, true);
    }

    #[test]
    fn case_uurcxstgmygtbstg() {
        let input = "uurcxstgmygtbstg";

        let result = is_really_nice(input);

        assert_eq!(result, false);
    }

    #[test]
    fn case_ieodomkazucvgmuy() {
        let input = "ieodomkazucvgmuy";

        let result = is_really_nice(input);

        assert_eq!(result, false);
    }
}
