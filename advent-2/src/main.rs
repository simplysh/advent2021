use std::fs;

fn main() {
    let filedata = fs::read_to_string("src/input.txt").expect("Read failed.");

    let mut distance = 0;
    let mut depth = 0;

    for line in filedata.lines() {
        let mut input = line.split(' ');

        let command = input.next().unwrap();
        let value: i32 = input.next().unwrap().parse().unwrap();

        match command {
            "forward" => {
                distance += value;
            },
            "up" => {
                depth -= value;
            }
            "down" => {
                depth += value;
            },
            _ => ()
        }
    }

    println!("{:<10}{:>10}", "distance:", distance);
    println!("{:<10}{:>10}", "depth:", depth);
    println!("{:<10}{:>10}", "total:", distance * depth);
}
