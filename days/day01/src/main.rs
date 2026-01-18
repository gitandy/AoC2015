use std::fs::File;
use std::io::Read;

fn main() {
    let input = File::open("data/input.txt");
    let mut buffer = String::new();
    input.unwrap().read_to_string(&mut buffer).expect("Couldn't read input file");

    let floor = eval_floor(&buffer);
    println!("Calculated floor #{floor}");

    let fb = first_basement(&buffer);
    println!("First char for basement #{fb}");
}

fn eval_floor(buffer: &String) -> i32 {
    let mut floor = 0;

    for c in buffer.clone().chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    floor
}

fn first_basement(buffer: &String) -> u32{
    let mut floor = 0;
    let mut char_count = 0;

    for c in buffer.clone().chars() {
        char_count += 1;

        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            break;
        }
    }

    char_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(eval_floor(&"(())".to_string()), 0);
        assert_eq!(eval_floor(&"()()".to_string()), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(eval_floor(&"(((".to_string()), 3);
        assert_eq!(eval_floor(&"(()(()(".to_string()), 3);
        assert_eq!(eval_floor(&"))(((((".to_string()), 3);
    }

    #[test]
    fn test_m1() {
        assert_eq!(eval_floor(&"())".to_string()), -1);
        assert_eq!(eval_floor(&"))(".to_string()), -1);
    }

    #[test]
    fn test_m3() {
        assert_eq!(eval_floor(&")))".to_string()), -3);
        assert_eq!(eval_floor(&")())())".to_string()), -3);
    }

    #[test]
    fn test_fb() {
        assert_eq!(first_basement(&")".to_string()), 1);
        assert_eq!(first_basement(&"()())".to_string()), 5);
    }
}