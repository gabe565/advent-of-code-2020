use group::Group;

mod group;

const EXAMPLE_INPUT: &'static str = include_str!("example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn part1(input :&'static str) -> u32 {
    input.split("\n\n")
        .map(|i| Group::new(i).some_answered_yes().len() as u32)
        .sum()
}

fn part2(input :&'static str) -> u32 {
    input.split("\n\n")
        .map(|i| Group::new(i).all_answered_yes().len() as u32)
        .sum()
}

pub fn main() {
    println!("6.1 Example: {}", part1(EXAMPLE_INPUT));
    println!("6.1 Problem: {}", part1(PROBLEM_INPUT));
    println!("6.2 Example: {}", part2(EXAMPLE_INPUT));
    println!("6.2 Problem: {}", part2(PROBLEM_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_works() {
        assert_eq!(part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn problem_1_works() {
        assert_eq!(part1(PROBLEM_INPUT), 6703);
    }

    #[test]
    fn example_2_works() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn problem_2_works() {
        assert_eq!(part2(PROBLEM_INPUT), 3430);
    }
}
