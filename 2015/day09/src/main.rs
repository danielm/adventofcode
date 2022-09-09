use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

use regex::Regex;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    // Vector of city names
    let mut cities: Vec<String> = Vec::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    let re = Regex::new(r"(?P<start>\w+) to (?P<finish>\w+) = (?P<distance>\d+)").unwrap();

    for row in reader.lines() {
        let instruction = row.unwrap();
        let cap = re.captures(instruction.as_str()).expect("Cannot parse instruction");

        let start = cap.name("start").expect("Failed to get Start").as_str();
        let finish = cap.name("finish").expect("Failed to get Finish").as_str();
        let distance = cap.name("distance").expect("Failed to get Distance").as_str();

        if !cities.contains(&start.to_string()) {
            cities.push(start.to_string());
        }

        if !cities.contains(&finish.to_string()) {
            cities.push(finish.to_string());
        }

        distances.insert((start.to_string(), finish.to_string()), distance.parse().unwrap());
    }

    let min = solve(&cities, &distances);

    println!("P1> Min Distance: {}", min);

    Ok(())
}

fn calculate(distances: &HashMap<(String, String), u32>, from: String, to: String) -> u32 {
    let mut route = (from.to_string(), to.to_string());

    if distances.contains_key(&route) {
        return *distances.get(&route).unwrap();
    }

    route = (to.to_string(), from.to_string());
    return *distances.get(&route).unwrap();
}

fn solve(cities: &Vec<String>, distances: &HashMap<(String, String), u32>) -> u32 {
    let cities_count = cities.len();

    let mut total: Vec<u32> = Vec::new();

    for perm in cities.iter().permutations(cities_count) {

        let distance = perm.windows(2)
            .map(|pair| calculate(distances, (&pair[0]).to_string(), (&pair[1]).to_string()))
            .sum();

        total.push(distance);
    }

    *total.iter().min().unwrap()
}

///:w
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
        let cities: Vec<String> = vec!["London".to_string(), "Dublin".to_string(), "Belfast".to_string()];
        let distances = HashMap::from([
            ((String::from("London"),String::from("Dublin")), 464),
            ((String::from("London"),String::from("Belfast")), 518),
            ((String::from("Dublin"),String::from("Belfast")), 141),
        ]);

        let solution = solve(&cities, &distances);

        assert_eq!(solution, 605);
    }
}
