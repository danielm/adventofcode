use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Light {
    status: bool,
    brightness: u32,
}

impl Light {
    fn new() -> Light {
        Light { status: false, brightness: 0 }
    }

    fn turn_on(&mut self) {
        self.status = true;
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        self.status = false;

        if self.brightness > 0 {
            self.brightness -= 1;
        }
    }

    fn toggle(&mut self) {
        self.status = !self.status;
        self.brightness += 2;
    }
}

fn main() -> io::Result<()> {
    let mut grid = init_grid(1000, 1000);

    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    for row in reader.lines() {
        run_instruction(&mut grid, row.unwrap().as_str());
    }

    let result = get_lights_report(&grid);

    println!("P1> Lights on: {}", result.0);
    println!("P2> Total Brightness: {}", result.1);

    Ok(())
}

fn run_instruction(grid: &mut Vec<Vec<Light>>, instruction: &str) {
    let re = Regex::new(r"^(?P<cmd>turn on|turn off|toggle) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)$").unwrap();

    let cap = re.captures(instruction).expect("Failed to parse the intruction");

    // process results
    let cmd = cap.name("cmd").unwrap().as_str();
    let x1: usize = cap.name("x1").unwrap().as_str().parse().unwrap();
    let y1: usize = cap.name("y1").unwrap().as_str().parse().unwrap();
    let x2: usize = cap.name("x2").unwrap().as_str().parse().unwrap();
    let y2: usize = cap.name("y2").unwrap().as_str().parse().unwrap();
  
    // All matching lights
    for i in x1..=x2 {
        for j in y1..=y2 {
            match cmd {
                "turn on" => grid[i][j].turn_on(),
                "turn off" => grid[i][j].turn_off(),
                "toggle" => grid[i][j].toggle(),
                _ => panic!("unsupported command")
            }

        }
    }
}

fn init_grid(x: i32, y: i32) -> Vec<Vec<Light>> {
    let mut grid: Vec<Vec<Light>> = Vec::new();

    for _i in 0..x {
        let mut row: Vec<Light> = Vec::new();

        for _j in 0..y {
           row.push(Light::new()); 
        }

        grid.push(row);
    }

    return grid;
}

fn get_lights_report(grid: &Vec<Vec<Light>>) -> (u32, u32) {
    let mut counter: u32 = 0;
    let mut brightness: u32 = 0;

    for row in grid {
        for light in row {
            if light.status {
                counter += 1;
            }

            brightness += light.brightness;
        }
    }

    return (counter, brightness);
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
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "turn on 0,0 through 999,999");

        let result = get_lights_report(&grid);

        assert_eq!(result.0, 1000000);
    }

    #[test]
    fn case_2() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "toggle 0,0 through 999,0");

        let result = get_lights_report(&grid);

        assert_eq!(result.0, 1000);
    }

    #[test]
    fn case_3() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "turn off 499,499 through 500,500");

        let result = get_lights_report(&grid);

        assert_eq!(result.0, 0);
    }

    #[test]
    fn case_4() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "turn on 0,0 through 0,0");

        let result = get_lights_report(&grid);

        assert_eq!(result.1, 1);
    }

    #[test]
    fn case_5() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "toggle 0,0 through 999,999");

        let result = get_lights_report(&grid);

        assert_eq!(result.1, 2000000);
    }
}
