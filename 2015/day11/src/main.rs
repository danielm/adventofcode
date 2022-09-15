/**
 * I.... really had a bad time with this one....... excuse this solution probably, it just does the
 * job
 *
 * Probably need some serious optimization, like avoid unnecesary iterations and better ways
 * to find pairs / straights.... Be ... my.... guess....
 */

fn main() {
    let next = next_password("hxbxwxba");

    println!("P1> Next password is: {}", next);
    println!("P2> And the Next password is: {}", next_password(next.as_str()));
}

fn next_password(input: &str) -> String {
    let mut current = String::from(input);

    // let mut i = 0;
    loop {
        current = increment_password(current);

        if is_valid(current.as_str()) {
            break;
        }

        // i +=1;
    }

    return current;
}

fn is_valid(current: &str) -> bool {
    let input = String::from(current).into_bytes();

    !has_banned_chars(&input) && has_pairs(&input) && has_straight(&input)
}

fn has_pairs(input: &[u8]) -> bool {
    let mut i = 0;
    let mut found_pairs: Vec<u8> = Vec::new();

    while i < input.len() - 1 {
        if input[i] == input[i + 1] && !found_pairs.contains(&input[i]) {
            found_pairs.push(input[i]);
            i += 1; // to avoid overlapping
        }

        i += 1;
    }

    found_pairs.len() >= 2
}

fn has_straight(input: &[u8]) -> bool {
    input.windows(3)
        .any(|c| {
            c[0] + 1 == c[1] && c[1] + 1 == c[2]
        })
}

fn has_banned_chars(input: &[u8]) -> bool {
    // 'l' || 'i' || 'o'
    input.contains(&108) || input.contains(&105) || input.contains(&111)
}

fn increment_password(current: String) -> String {
    let mut input = current.into_bytes();
    let mut i = input.len();

    while i > 0 {
        i -= 1;
        
        if input[i] as char == 'z' {
            input[i] = 'a' as u8;
        } else {
            input[i] += 1;
            break;
        }
    }

    input.iter().map(|c| *c as char).collect::<String>()
}

///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn case_1() {
        assert_eq!(false, is_valid("hijklmmn"));
    }

    #[test]
    fn case_2() {
        assert_eq!(false, is_valid("abbceffg"));
    }


    #[test]
    fn case_3() {
        assert_eq!(false, is_valid("abbcegjk"));
    }

    #[test]
    fn case_4() {
        let result = next_password("abcdefgh");

        assert_eq!(result, "abcdffaa");
    }

    #[test]
    fn case_5() {
        let result = next_password("ghijklmn");

        assert_eq!(result, "ghjaabcc");
    }
}
