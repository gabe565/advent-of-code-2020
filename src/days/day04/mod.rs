use passport::Passport;

mod passport;

const PART1_EXAMPLE_INPUT: &'static str = include_str!("part1_example.txt");
const PART2_INVALID_INPUT: &'static str = include_str!("part2_invalid_example.txt");
const PART2_VALID_INPUT: &'static str = include_str!("part2_valid_example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn part1(input: &'static str) -> u32 {
    let mut count = 0;
    for i in input.split("\n\n") {
        let passport = Passport::from_str(i);
        if passport.part1_is_valid() {
            count += 1;
        }
    }
    count
}

fn part2(input: &'static str) -> u32 {
    let mut count = 0;
    for i in input.split("\n\n") {
        let passport = Passport::from_str(i);
        if passport.part2_is_valid() {
            count += 1;
        }
    }
    count
}

pub fn main() {
    println!("4.1 Example: {}", part1(PART1_EXAMPLE_INPUT));
    println!("4.1 Problem: {}", part1(PROBLEM_INPUT));
    println!("4.2 Invalid Example: {}", part2(PART2_INVALID_INPUT));
    println!("4.2 Valid Example: {}", part2(PART2_VALID_INPUT));
    println!("4.2 Problem: {}", part2(PROBLEM_INPUT));
}

#[test]
fn test_example_1() {
    assert_eq!(part1(PART1_EXAMPLE_INPUT), 2);
}

#[test]
fn test_example_2_invalid() {
    assert_eq!(part2(PART2_INVALID_INPUT), 0);
}

#[test]
fn test_example_2_valid() {
    assert_eq!(part2(PART2_VALID_INPUT), 4);
}