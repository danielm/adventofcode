use md5::{Md5,Digest};
use std::str;

fn main() {
    let secret = "ckczppom";

    let mut coin = mine_adventcoin(secret, "00000");

    println!("Solution P1 is: {}", coin);

    coin = mine_adventcoin(secret, "000000");

    println!("Solution P2 is: {}", coin);
}

fn mine_adventcoin(secret: &str, match_check: &str) -> i32
{
    let mut solution: i32 = 0;

    loop {
        let input = format!("{}{}", secret, solution);

        let mut hasher = Md5::new();
        hasher.update(input);

        let result = hasher.finalize();

        let signature = hex::encode(&result[..]);

        if signature.starts_with(match_check) {
            break;
        }

        solution += 1;
    }

    return solution;
}

///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use crate::{mine_adventcoin};

    #[test]
    fn case_abcdef() {
        let result = mine_adventcoin("abcdef", "00000");

        assert_eq!(result, 609043);
    }

    #[test]
    fn case_pqrstuv() {
        let result = mine_adventcoin("pqrstuv", "00000");

        assert_eq!(result, 1048970);
    }
}
