use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Reads the input for the exercise
pub fn read_input<T: AsRef<Path>>(path: T) -> String {
    let input = File::open(path);
    let mut buffer = String::new();
    input
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("Couldn't read input file");
    buffer
}

/// Describes the first or second part to run
#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two,
}

/// If program is run with parameter "two" the second part should be executed
pub fn get_part() -> Part {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front(); // Remove executable

    if let Some(part) = args.pop_front()
        && part == "two"
    {
        Part::Two
    } else {
        Part::One
    }
}
