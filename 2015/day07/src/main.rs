use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);
    
    let re = Regex::new(r"(?P<input>.*)\s->\s(?P<output>\w+)").unwrap();

    let mut gates: HashMap<String, String> = HashMap::new();

    for row in reader.lines() {
        let instruction = row.expect("Cannot read line");

        let cap = re.captures(instruction.as_str()).expect("Cannot parse instruction");
        let output = cap.name("output").expect("Failed to get Output").as_str();
        let input = cap.name("input").expect("Failed to get Input").as_str();       
        
        gates.insert(String::from(output), String::from(input));
    }

    println!("P1> Signal at 'a' is {}",get_signal(&mut gates, "a"));

    Ok(())
}

fn get_signal(gates: &mut HashMap<String, String>, wire: &str) -> u16 {
    let gate = gates[wire].clone();

    let re = Regex::new(r"^((?P<feed>\S+)|((?P<left>\S+)\s(?P<operation>AND|OR|LSHIFT|RSHIFT)\s(?P<right>\S+))|(NOT\s(?P<not>\S+)))$").unwrap();

    let cap = re.captures(&gate).expect("Cannot parse wire"); 
    let mut result: u16 = 0;

    if let Some(feed) = cap.name("feed") {
        // 123 -> a
        let value = feed.as_str();

        result = match get_digit(value) {
            Some(v) => v,
            None => get_signal(gates, value)
        };
    } else if let Some(operation) = cap.name("operation") {
        // x AND y -> z
        let left = cap.name("left").unwrap().as_str();
        let right = cap.name("right").unwrap().as_str();

        let left = match get_digit(left) {
            Some(v) => v,
            None => get_signal(gates, left),
        };

        let right = match get_digit(right) {
            Some(v) => v,
            None => get_signal(gates, right),
        };

        result = match operation.as_str() {
            "AND" => left & right,
            "OR" => left | right,
            "RSHIFT" => left >> right,
            "LSHIFT" => left << right,
            _ => panic!("Unkown operation {}", operation.as_str())
         };
    } else if let Some(not) = cap.name("not") {
        // NOT e -> f
        
        result = match get_digit(not.as_str()) {
            Some(v) => !v,
            None => !get_signal(gates, not.as_str())
        };
    }

    // Replace operation with the result
    *gates.get_mut(wire).unwrap() = result.to_string();

    return result;
}

fn get_digit(input: &str) -> Option<u16> {
    if input.chars().all(|x| x.is_digit(10)) {
        return Some(input.to_string().parse::<u16>().unwrap());
    }
    None
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
        let mut gates: HashMap<String, String> = HashMap::from([
            ("x".to_string(), "123".to_string()),
            ("y".to_string(), "456".to_string()),
            ("d".to_string(), "x AND y".to_string()),
            ("e".to_string(), "x OR y".to_string()),
            ("f".to_string(), "x LSHIFT 2".to_string()),
            ("g".to_string(), "y RSHIFT 2".to_string()),
            ("h".to_string(), "NOT x".to_string()),
            ("i".to_string(), "NOT y".to_string()),
        ]);

        let d = get_signal(&mut gates, "d");
        let e = get_signal(&mut gates, "e");
        let f = get_signal(&mut gates, "f");
        let g = get_signal(&mut gates, "g");
        let h = get_signal(&mut gates, "h");
        let i = get_signal(&mut gates, "i");
        let x = get_signal(&mut gates, "x");
        let y = get_signal(&mut gates, "y");

        assert_eq!(d, 72);
        assert_eq!(e, 507);
        assert_eq!(f, 492);
        assert_eq!(g, 114);
        assert_eq!(h, 65412);
        assert_eq!(i, 65079);
        assert_eq!(x, 123);
        assert_eq!(y, 456);
    }
}
