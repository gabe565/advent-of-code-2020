use password::Password;

mod password;

const EXAMPLE_INPUT: &'static str = include_str!("example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn part1(input: &'static str) -> usize {
    input.split("\n")
        .map(|i| Password::from(i).part1_valid())
        .filter(|i| i == &true)
        .count()
}

fn part2(input: &'static str) -> usize {
    input.split("\n")
        .map(|i| Password::from(i).part2_valid())
        .filter(|i| i == &true)
        .count()
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