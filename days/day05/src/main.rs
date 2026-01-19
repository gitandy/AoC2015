use util::read_input;

#[derive(PartialEq, Debug)]
enum Quality {
    Nice,
    Naughty,
}

const BAD_CHARS: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: &str = "aeiou";

fn main() {
    let buffer = read_input("days/day05/data/input.txt");

    let nice_words = buffer
        .lines()
        .map(|word| check_word(word))
        .filter(|i| i == &Quality::Nice)
        .count();

    println!("Counted #{nice_words} nice words");
}

fn check_word(word: &str) -> Quality {
    if BAD_CHARS
        .iter()
        .filter(|p| word.contains(**p))
        .count()
        > 0
    {
        return Quality::Naughty;
    }

    let vowels = word.chars().filter(|c| VOWELS.contains(*c)).count();
    let mut chars = word.chars().into_iter().collect::<Vec<char>>();
    chars.dedup();

    if vowels >= 3 && word.len() - chars.len() > 0 {
        Quality::Nice
    } else {
        Quality::Naughty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naughty() {
        assert_eq!(check_word("jchzalrnumimnmhp"), Quality::Naughty);
        assert_eq!(check_word("haegwjzuvuyypxyu"), Quality::Naughty);
        assert_eq!(check_word("dvszwmarrgswjxmb"), Quality::Naughty);
    }

    #[test]
    fn test_nice() {
        assert_eq!(check_word("ugknbfddgicrmopn"), Quality::Nice);
        assert_eq!(check_word("aaa"), Quality::Nice);
    }
}
