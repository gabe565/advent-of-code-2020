use boarding_pass::BoardingPass;

mod boarding_pass;

const EXAMPLE_1_INPUT: &'static str = "FBFBBFFRLR";
const EXAMPLE_2_INPUT: &'static str = "BFFFBBFRRR";
const EXAMPLE_3_INPUT: &'static str = "FFFBBBFRRR";
const EXAMPLE_4_INPUT: &'static str = "BBFFBBFRLL";
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

pub fn part1(input: &'static str) -> Option<BoardingPass> {
    input.split("\n")
        .map(|i| BoardingPass::from(i))
        .max()
}

pub fn part2(input: &'static str) -> Option<u32> {
    let mut passes = input.split("\n")
        .map(|i| BoardingPass::from(i))
        .collect::<Vec<BoardingPass>>();

    passes.sort();

    let mut prev: Option<&BoardingPass> = None;
    for pass in &passes {
        if let Some(prev) = prev {
            if prev.seat() != pass.seat() - 1 {
                return Some(pass.seat() - 1);
            }
        }
        prev = Some(pass);
    }
    None
}

pub fn main() {
    println!("5.1 Example 1: {}", BoardingPass::from(EXAMPLE_1_INPUT));
    println!("5.1 Example 2: {}", BoardingPass::from(EXAMPLE_2_INPUT));
    println!("5.1 Example 3: {}", BoardingPass::from(EXAMPLE_3_INPUT));
    println!("5.1 Example 4: {}", BoardingPass::from(EXAMPLE_4_INPUT));
    println!("5.1 Problem: {}", part1(PROBLEM_INPUT).unwrap());
    println!("5.2 Problem: seat ID {}", part2(PROBLEM_INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_boarding_pass_eq(pass: &BoardingPass, row: u32, col: u32, seat: u32) {
        assert_eq!(pass.row, row);
        assert_eq!(pass.col, col);
        assert_eq!(pass.seat(), seat);
    }

    #[test]
    pub fn example_1_works() {
        assert_boarding_pass_eq(&BoardingPass::from(EXAMPLE_1_INPUT), 44, 5, 357);
    }

    #[test]
    pub fn example_2_works() {
        assert_boarding_pass_eq(&BoardingPass::from(EXAMPLE_2_INPUT), 70, 7, 567);
    }

    #[test]
    pub fn example_3_works() {
        assert_boarding_pass_eq(&BoardingPass::from(EXAMPLE_3_INPUT), 14, 7, 119);
    }

    #[test]
    pub fn example_4_works() {
        assert_boarding_pass_eq(&BoardingPass::from(EXAMPLE_4_INPUT), 102, 4, 820);
    }
}
