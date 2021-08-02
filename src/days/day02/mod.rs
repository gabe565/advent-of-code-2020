use password::Password;

mod password;

const EXAMPLE_INPUT: &'static str = include_str!("example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input.split("\n").collect()
}

fn part1(input: &'static str) -> i32 {
    let mut count = 0;
    for i in parse_input(input) {
        let password = Password::from_str(i);
        if password.part1_valid() {
            count += 1;
        }
    }
    count
}

fn part2(input: &'static str) -> i32 {
    let mut count = 0;
    for i in parse_input(input) {
        let password = Password::from_str(i);
        if password.part2_valid() {
            count += 1;
        }
    }
    count
}

pub fn main() {
    println!("2.1 Example: {}", part1(EXAMPLE_INPUT));
    println!("2.1 Problem: {}", part1(PROBLEM_INPUT));
    println!("2.2 Example: {}", part2(EXAMPLE_INPUT));
    println!("2.2 Problem: {}", part2(PROBLEM_INPUT));
}

#[test]
fn test_example_1() {
    assert_eq!(part1(EXAMPLE_INPUT), 2);
}

#[test]
fn test_example_2() {
    assert_eq!(part2(EXAMPLE_INPUT), 1);
}