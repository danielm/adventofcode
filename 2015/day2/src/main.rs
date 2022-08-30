use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt").expect("Failed to open input");
    let reader = BufReader::new(file);

    let mut papper: i32 = 0;
    let mut ribbon_length: i32 = 0;

    for line in reader.lines() {
        let v: Vec<i32> = line?.split("x")
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        let l = v[0];
        let w = v[1];
        let h = v[2];

        let present = calc_wrap(l, w, h);
        let ribbon = calc_ribbon(l, w, h);

        papper += present;
        ribbon_length += ribbon;

        // println!("Surface of {}x{}x{} = {}", l, w, h, present);
    }

    println!("Papper: {}", papper);
    println!("Ribbon is: {}", ribbon_length);

    Ok(())
}

fn calc_ribbon(l: i32, w: i32, h: i32) -> i32 {
    let volume = l * w * h;

    let v = [l*2+h*2, w*2+h*2, l*2+w*2];

    return volume + *v.iter().min().unwrap();
}

fn calc_wrap(l: i32, w: i32, h: i32) -> i32 {
    calc_surface(l, w, h) + calc_extra(l, w, h)
}

// Area of the smallest side
fn calc_extra(l: i32, w: i32, h: i32) -> i32 {
    let v = [l*h, w*h, l*w];

    return *v.iter().min().unwrap();
}

// https://en.wikipedia.org/wiki/Cuboid#Rectangular_cuboid
fn calc_surface(l: i32, w: i32, h: i32) -> i32 {
    2*l*w + 2*w*h + 2*h*l
}


///
/// Testing bro
///
#[cfg(test)]
mod tests {
    use crate::{calc_extra, calc_surface};

    #[test]
    fn case_2x3x4() {
        let (l, w, h) = (2, 3, 4);

        let result = calc_surface(l, w, h) + calc_extra(l, w, h);

        assert_eq!(result, 58);
    }

    #[test]
    fn case_1x1x10() {
        let (l, w, h) = (1, 1, 10);

        let result = calc_surface(l, w, h) + calc_extra(l, w, h);

        assert_eq!(result, 43);
    }
}

