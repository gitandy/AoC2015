use std::fs::File;
use std::io::Read;

const ALTERNATE: bool = false;
const GRID_SIZE: usize = 160; // 160 works if Santa only, 150 works if both alternating

fn main() {
    let input = File::open("days/day03/data/input.txt");
    let mut buffer = String::new();
    input
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("Couldn't read input file");

    let (grid, least_once) = walk_grid(&mut buffer, ALTERNATE);
    println!("Houses with at least one present: {least_once} houses");

    for row in grid {
        println!("{row:?}");
    }
}

struct Point {
    pub x: usize,
    pub y: usize,
}

fn walk_grid(dirs: &mut String, alternate: bool) -> ([[u8; GRID_SIZE]; GRID_SIZE], u32) {
    let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];

    // Track pos of Santa and Robo
    let mut pos = [
        Point {
            x: GRID_SIZE / 2,
            y: GRID_SIZE / 2,
        },
        Point {
            x: GRID_SIZE / 2,
            y: GRID_SIZE / 2,
        },
    ];
    let mut a = 0;

    let mut least_once = 0;

    // Special init values for second part
    if alternate {
        grid[GRID_SIZE / 2][GRID_SIZE / 2] += 2;
        least_once = 1;
    }

    for dir in dirs.chars() {
        match dir {
            '^' => pos[a].x -= 1,
            'v' => pos[a].x += 1,
            '<' => pos[a].y -= 1,
            '>' => pos[a].y += 1,
            _ => panic!("Wrong direction char {dir}"),
        }
        grid[pos[a].x][pos[a].y] += 1;
        if grid[pos[a].x][pos[a].y] == 1 {
            least_once += 1;
        }

        a = if alternate && a == 0 { 1 } else { 0 };
    }

    (grid, least_once)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_only() {
        assert_eq!(walk_grid(&mut ">".to_string(), false).1, 1);
        assert_eq!(walk_grid(&mut "^>v<".to_string(), false).1, 4);
        assert_eq!(walk_grid(&mut "^v^v^v^v^v".to_string(), false).1, 2);
    }

    #[test]
    fn test_alternate() {
        assert_eq!(walk_grid(&mut "^v".to_string(), true).1, 3);
        assert_eq!(walk_grid(&mut "^>v<".to_string(), true).1, 3);
        assert_eq!(walk_grid(&mut "^v^v^v^v^v".to_string(), true).1, 11);
    }
}
