use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Regex;

use std::cmp;

#[derive(Default,Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    sprint: u32,
    rest: u32,

    current_position: u32,
    points: u32,
}

impl Reindeer {
    fn move_for(&mut self, seconds: u32) -> u32 {
        let step = self.sprint + self.rest;

        let cycles = seconds / step;
        let rest = seconds % step;

        let mut movingtime = cycles * self.sprint;

        if rest < self.sprint { movingtime += rest; } else { movingtime += self.sprint; }

        self.current_position = movingtime * self.speed;

        self.current_position
    }
}

fn race(reindeers: &mut Vec<Reindeer>, seconds: u32) -> u32 {
    let mut max = 0; 
    
    for i in 1..=seconds {
        max = 0;
        for deer in &mut *reindeers {
            max = cmp::max(max, deer.move_for(i));
        }

        for deer in &mut *reindeers {
            if deer.current_position == max {
                deer.points += 1;

                // println!("{} winner is {}", i, deer.name);
            }
        }
    }
    max
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"(?P<reindeer>\w+) can fly (?P<speed>\d+) km/s for (?P<sprint>\d+) seconds, but then must rest for (?P<rest>\d+) seconds\.").unwrap();

    let mut reindeers: Vec<Reindeer> = Vec::new();

    for row in reader.lines() {
        let instruction = row.unwrap();
        let cap = re.captures(instruction.as_str()).expect("Cannot parse instruction");

        let reindeer = cap.name("reindeer").expect("Failed to get reindeer").as_str();
        let speed = cap.name("speed").expect("Failed to get speed").as_str();
        let sprint = cap.name("sprint").expect("Failed to get sprint").as_str();
        let rest = cap.name("rest").expect("Failed to get reset").as_str();

        // println!("{} - speed: {} - sprint: {} - rest: {}", reindeer, speed, sprint, rest);
        
        reindeers.push(Reindeer {
            name: reindeer.to_string(),
            speed: speed.parse().unwrap(),
            sprint: sprint.parse().unwrap(),
            rest: rest.parse().unwrap(),
            ..Default::default()
        });
    }

    let result = race(&mut reindeers, 2503);
    println!("P1> Max distance: {}", result);

    //let winner = reindeers.iter().max_by_key(|r| r.current_position).unwrap();
    //println!("P2> Winner is: {}  wit {} pts", winner.name, winner.points);

    // println!("{:?}", winner);
    // println!("{:?}", reindeers);

    Ok(())
}
