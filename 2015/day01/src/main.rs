use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let mut f = BufReader::new(
        File::open("input.txt").expect("Cannot open file")
    );

    let mut floor: i32 = 0;

    let mut first_basement: i32 = 0;

    let mut buf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read Fail") != 0 {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("convertion fail");
        for (i, c) in s.chars().enumerate() {
           floor += match c {
                '(' => 1,
                ')' => -1,
                _ => 0
           };

           if floor < 0 && first_basement == 0 {
               first_basement = i as i32;
           }
        }
        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }

    println!("Result is: {}", floor);
    println!("First basement visit: {}", (first_basement + 1));
}
