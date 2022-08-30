use std::io::{BufRead, BufReader};
use std::fs::File;

struct Gifter {
    x: i32,
    y: i32,
}

impl Gifter {
    fn move_to(&mut self, direction: char) {
        match direction {
            '^' => self.y -= 1,
            'v' => self.y += 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!("Found something weird")
        }
    }

    fn get_house(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn main() {
    let part1 = count_houses(1);
    let part2 = count_houses(2);

    println!("P1 Solo House visited: {}", part1); 
    println!("P2 Assisted House visited: {}", part2); 
}

fn count_houses(n_gifters: i32) -> i32 {
    let mut f = BufReader::new(
        File::open("input.txt").expect("Cannot open file")
    );

    let mut houses_visited: Vec<(i32, i32)> = vec![(0, 0)];
    let mut gifters: Vec<Gifter> = vec![];
    let mut currentGifter = 0;

    // initialize gifters
    for i in 0..n_gifters {
        gifters.push(Gifter { x: 0, y: 0 });
    }

    let mut buf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read Fail") != 0 {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("convertion fail");
        for (i, c) in s.chars().enumerate() {
            
            gifters[currentGifter].move_to(c);

            let house = gifters[currentGifter].get_house();

            if !houses_visited.contains(&house) {
                houses_visited.push(house);
            }

            currentGifter += 1;
            if currentGifter >= gifters.len() {
                currentGifter = 0;
            }
        }
        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }

    return houses_visited.len() as i32;
}
