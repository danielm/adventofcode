use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

use regex::Regex;
use itertools::Itertools;

use std::cmp;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut attendees: Vec<String> = Vec::new();
    let mut friendships: HashMap<(String, String), i32> = HashMap::new();

    let re = Regex::new(r"(?P<person>\w+) would (?P<status>gain|lose) (?P<happiness>\d+) happiness units by sitting next to (?P<friend>\w+)\.").unwrap();

    for row in reader.lines() {
        let instruction = row.unwrap();
        let cap = re.captures(instruction.as_str()).expect("Cannot parse instruction");

        let person = cap.name("person").expect("Failed to get person").as_str();
        let friend = cap.name("friend").expect("Failed to get friend").as_str();
        let status = cap.name("status").expect("Failed to get status").as_str();
        let mut happiness: i32 = cap.name("happiness").expect("Failed to get hapiness").as_str().parse().unwrap();

        if status == "lose" {
            happiness *= -1;
        }

        // println!("{} => {}: {}", person, friend, happiness);

        if !attendees.contains(&person.to_string()) {
            attendees.push(person.to_string());
        }

        friendships.insert((person.to_string(), friend.to_string()), happiness);
    }

    let mut result = solve(&attendees, &friendships);
    println!("P1> Max happiness arrangement is: {}", result);

    // Add myself
    let y0 = "Me";
    attendees.push(y0.to_string());
    for person in &attendees {
        friendships.insert((y0.to_string(), person.to_string()), 0);
        friendships.insert((person.to_string(), y0.to_string()), 0);
    }

    result = solve(&attendees, &friendships);
    println!("P2> Max happiness arrangement w/myself is: {}", result);

    Ok(())
}

fn calculate(friendships: &HashMap<(String, String), i32>, person: String, friend: String) -> i32 {
    *friendships.get(&(person.to_string(), friend.to_string())).unwrap() + *friendships.get(&(friend.to_string(), person.to_string())).unwrap()
}

fn solve(attendees: &Vec<String>, friendships: &HashMap<(String, String), i32>) -> i32 {
    let attendees_count = attendees.len();

    let mut max = 0;

    // Review: This does the job, but produces 2x iterations because:
    // [A,B,C] != [C, B, A]
    // Can we skip this using itertools?
    for arrangement in attendees.iter().permutations(attendees_count) {
        let mut happiness = arrangement.windows(2)
            .map(|pair| calculate(friendships, (&pair[0]).to_string(), (&pair[1]).to_string()))
            .sum();

        // Add up the happiness of first and last element because its a circle
        happiness += calculate(friendships,
            arrangement.first().unwrap().to_string(),
            arrangement.last().unwrap().to_string()
        );
        
        max = cmp::max(max, happiness);
    }

    max
}
