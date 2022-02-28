use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
struct Diagnostic(i32, i32);

impl Diagnostic {
    fn new() -> Diagnostic {
        Diagnostic(0, 0)
    }

    fn count(&mut self, bit: char) {
        if bit == '1' {
            self.0 += 1;
        } else {
            self.1 += 1;
        }
    }

    fn gamma(&self) -> String {
        let Diagnostic(set, unset) = self;
        match set.cmp(unset) {
            Ordering::Greater => String::from("1"),
            _ => String::from("0"),
        }
    }
    fn epsilon(&self) -> String {
        let Diagnostic(set, unset) = self;
        match set.cmp(unset) {
            Ordering::Less => String::from("1"),
            _ => String::from("0"),
        }
    }
}

fn main() {
    let filedata = fs::read_to_string("src/input.txt").expect("Failed to read.");
    let mut report: Vec<Diagnostic> = Vec::new();

    for line in filedata.lines() {
        line.chars().enumerate().for_each(|(offset, bit)| {
            match report.get_mut(offset) {
                Some(diag) => {
                    diag.count(bit);
                },
                None => {
                    let mut diag = Diagnostic::new();
                    diag.count(bit);
                    report.push(diag);
                }
            };
        });
    }

    for diag in &report {
        println!("{:?}", diag);
    }

    let gamma: String = report.iter().fold(String::new(), |acc, diag| acc + &diag.gamma());
    let epsilon: String = report.iter().fold(String::new(), |acc, diag| acc + &diag.epsilon());

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);

    let gamma = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("power consumption: {}", gamma * epsilon);
}
