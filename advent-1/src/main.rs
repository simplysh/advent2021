use std::fs;
use std::cmp::Ordering;

fn main() {
    let filedata = fs::read_to_string("src/input.txt").expect("Can't read!");

    let mut last: Option<i32> = None;
    let mut increase: u32 = 0;

    for line in filedata.lines() {
        let depth: i32 = line.trim().parse().expect("Not a number!");

        increase += match depth.cmp(&last.unwrap_or(depth)) {
            Ordering::Greater => 1,
            _ => 0
        };

        last = Some(depth);
    }

    println!("Increased {} times", increase);
}
