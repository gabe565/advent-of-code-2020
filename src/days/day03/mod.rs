use map::Map;

mod map;

const EXAMPLE_INPUT: &'static str = include_str!("example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn process(map: &mut Map, x: usize, y: usize) -> u32 {
    let mut count = 0;
    while !map.at_bottom() {
        map.move_xy(x, y);
        if map.on_tree() {
            count += 1;
        }
    }
    count
}

fn part1(input: &'static str) -> u32 {
    process(&mut Map::from(input), 3, 1)
}

fn part2(input: &'static str) -> u32 {
    let mut map = Map::from(input);
    let mut result: u32 = 1;
    for [x, y] in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
        result *= process(&mut map, x, y) as u32;
        map.reset();
    }
    result
}

pub fn main() {
    println!("3.1 Example: {}", part1(EXAMPLE_INPUT));
    println!("3.1 Problem: {}", part1(PROBLEM_INPUT));
    println!("3.2 Example: {}", part2(EXAMPLE_INPUT));
    println!("3.2 Problem: {}", part2(PROBLEM_INPUT));
}

#[test]
fn test_example_1() {
    assert_eq!(part1(EXAMPLE_INPUT), 7);
}

#[test]
fn test_example_2() {
    assert_eq!(part2(EXAMPLE_INPUT), 336);
}