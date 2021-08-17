use boarding_pass::BoardingPass;
use seat::Seat;

mod boarding_pass;
mod seat;

const EXAMPLE_1_INPUT: &'static str = "FBFBBFFRLR";
const EXAMPLE_2_INPUT: &'static str = "BFFFBBFRRR";
const EXAMPLE_3_INPUT: &'static str = "FFFBBBFRRR";
const EXAMPLE_4_INPUT: &'static str = "BBFFBBFRLL";
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

pub fn part1(input: &'static str) -> Option<BoardingPass> {
    input.split("\n")
        .map(|i| BoardingPass::from_str(i))
        .max()
}

pub fn part2(input: &'static str) -> Option<Seat> {
    let mut passes = input.split("\n")
        .map(|i| BoardingPass::from_str(i))
        .collect::<Vec<BoardingPass>>();

    passes.sort();

    let mut prev: Option<&BoardingPass> = None;
    for pass in &passes {
        if let Some(prev) = prev {
            if prev.seat.id != pass.seat.id - 1 {
                return Some(Seat::new(pass.seat.id - 1));
            }
        }
        prev = Some(pass);
    }
    None
}

pub fn main() {
    println!("5.1 Example 1: {}", BoardingPass::from_str(EXAMPLE_1_INPUT));
    println!("5.1 Example 2: {}", BoardingPass::from_str(EXAMPLE_2_INPUT));
    println!("5.1 Example 3: {}", BoardingPass::from_str(EXAMPLE_3_INPUT));
    println!("5.1 Example 4: {}", BoardingPass::from_str(EXAMPLE_4_INPUT));
    println!("5.1 Problem: {}", part1(PROBLEM_INPUT).unwrap());
    println!("5.2 Problem: {}", part2(PROBLEM_INPUT).unwrap());
}

#[test]
pub fn test_example_1() {
    let pass = BoardingPass::from_str(EXAMPLE_1_INPUT);
    assert_eq!(pass.row, 44);
    assert_eq!(pass.col, 5);
    assert_eq!(pass.seat.id, 357);
}

#[test]
pub fn test_example_2() {
    let pass = BoardingPass::from_str(EXAMPLE_2_INPUT);
    assert_eq!(pass.row, 70);
    assert_eq!(pass.col, 7);
    assert_eq!(pass.seat.id, 567);
}

#[test]
pub fn test_example_3() {
    let pass = BoardingPass::from_str(EXAMPLE_3_INPUT);
    assert_eq!(pass.row, 14);
    assert_eq!(pass.col, 7);
    assert_eq!(pass.seat.id, 119);
}

#[test]
pub fn test_example_4() {
    let pass = BoardingPass::from_str(EXAMPLE_4_INPUT);
    assert_eq!(pass.row, 102);
    assert_eq!(pass.col, 4);
    assert_eq!(pass.seat.id, 820);
}