use std::fs::File;
use std::io::Read;

fn main() {
    let input = File::open("data/input.txt");
    let mut buffer = String::new();
    input
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("Couldn't read input file");

    let mut paper_sq: u32 = 0;
    let mut ribbon_len: u32 = 0;

    for l in buffer.lines() {
        let mut size = l.split('x')
            .map(|i| i.parse::<u32>().unwrap_or(0));

        let (width, height, length) =
            if let (Some(w), Some(h), Some(l)) = (size.next(), size.next(), size.next()) {
                (w, h, l)
            } else {
                (0, 0, 0)
            };

        paper_sq += eval_paper_area(width, height, length);
        ribbon_len += eval_ribbon_len(width, height, length);
    }

    println!("Needed paper {paper_sq} sqfeet");
    println!("Needed ribbon {ribbon_len} feet");
}

fn eval_paper_area(width: u32, height: u32, length: u32) -> u32 {
    let sides = vec![width*height, height*length, width*length];
    let smallest = if let Some(x) = sides.iter().min() {x} else {&0};
    sides.iter().sum::<u32>() * 2u32 + smallest
}

fn eval_ribbon_len(width: u32, height: u32, length: u32) -> u32 {
    let perimeter = vec![width+height, height+length, width+length];
    let smallest = if let Some(x) = perimeter.iter().min() {x} else {&0};
    2 * smallest + (width*height*length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big() {
        assert_eq!(eval_paper_area(2,3,4), 58);
        assert_eq!(eval_ribbon_len(2,3,4), 34);
    }

    #[test]
    fn test_small() {
        assert_eq!(eval_paper_area(1,1,10), 43);
        assert_eq!(eval_ribbon_len(1,1,10), 14);
    }
}
