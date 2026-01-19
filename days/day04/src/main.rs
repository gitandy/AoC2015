use md5::{Digest, Md5};
use std::io::Write;

const SECRET: &str = "iwrupvqb";
const ZEROS: u8 = 6;

fn main() {
    let zeros = &String::from("0").repeat(ZEROS as usize);

    for i in 0..10_000_000 {
        let hash = proc_md5(SECRET, i);
        if hash.starts_with(zeros) {
            println!("\nHash is: {hash} with Nonce #{i}");
            break;
        }

        // Give a lifesign
        if i % 10_u32.pow(ZEROS as u32 - 1) as usize == 0 {
            print!(".");
            let _ = std::io::stdout().flush();
        }
    }
}

fn proc_md5(secret: &str, nr: usize) -> String {
    let mut hasher = Md5::new();
    hasher.update(secret);
    hasher.update(format!("{nr}"));

    format!("{:032x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert!(proc_md5("abcdef", 609043).starts_with("00000"));
        assert!(proc_md5("pqrstuv", 1048970).starts_with("00000"));
    }
}
