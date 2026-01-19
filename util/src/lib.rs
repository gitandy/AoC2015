use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input<T: AsRef<Path>>(path: T) -> String {
    let input = File::open(path);
    let mut buffer = String::new();
    input
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("Couldn't read input file");
    buffer
}
