use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// TODO: instead of bool, lets use a i32
// -> starts at 0

fn main() -> io::Result<()> {
    let mut grid = init_grid(1000, 1000);

    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    for row in reader.lines() {
        run_instruction(&mut grid, row.unwrap().as_str());
    }

    let result = count_lights_on(&grid);

    println!("P1> Lights on: {}", result);

    Ok(())
}

fn run_instruction(grid: &mut Vec<Vec<bool>>, instruction: &str) {
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
                "turn on" => grid[i][j] = true,
                "turn off" => grid[i][j] = false,
                "toggle" => grid[i][j] = !grid[i][j],
                _ => panic!("unsupported command")
            }

        }
    }
}

fn init_grid(x: i32, y: i32) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for _i in 0..x {
        let mut row: Vec<bool> = Vec::new();

        for _j in 0..y {
           row.push(false); 
        }

        grid.push(row);
    }

    return grid;
}

fn count_lights_on(grid: &Vec<Vec<bool>>) -> i32 {
    let mut counter: i32 = 0;

    for row in grid {
        for light in row {
            if *light {
                counter += 1;
            }
        }
    }

    return counter;
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

        let result = count_lights_on(&grid);

        assert_eq!(result, 1000000);
    }

    #[test]
    fn case_2() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "toggle 0,0 through 999,0");

        let result = count_lights_on(&grid);

        assert_eq!(result, 1000);
    }

    #[test]
    fn case_3() {
        let mut grid = init_grid(1000, 1000);

        run_instruction(&mut grid, "turn off 499,499 through 500,500");

        let result = count_lights_on(&grid);

        assert_eq!(result, 0);
    }
}
