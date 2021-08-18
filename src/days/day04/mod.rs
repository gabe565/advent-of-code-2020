use passport::Passport;

mod passport;

const PART1_EXAMPLE_INPUT: &'static str = include_str!("part1_example.txt");
const PART2_INVALID_INPUT: &'static str = include_str!("part2_invalid_example.txt");
const PART2_VALID_INPUT: &'static str = include_str!("part2_valid_example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");

fn part1(input: &'static str) -> usize {
    input.split("\n\n")
        .map(|i| Passport::from(i).part1_is_valid())
        .filter(|i| i == &true)
        .count()
}

fn part2(input: &'static str) -> usize {
    input.split("\n\n")
        .map(|i| Passport::from(i).part2_is_valid())
        .filter(|i| i == &true)
        .count()
}

pub fn main() {
    println!("4.1 Example: {}", part1(PART1_EXAMPLE_INPUT));
    println!("4.1 Problem: {}", part1(PROBLEM_INPUT));
    println!("4.2 Invalid Example: {}", part2(PART2_INVALID_INPUT));
    println!("4.2 Valid Example: {}", part2(PART2_VALID_INPUT));
    println!("4.2 Problem: {}", part2(PROBLEM_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_works() {
        assert_eq!(part1(PART1_EXAMPLE_INPUT), 2);
    }

    #[test]
    fn problem_1_works() {
        assert_eq!(part1(PROBLEM_INPUT), 254);
    }

    #[test]
    fn example_2_invalid_works() {
        assert_eq!(part2(PART2_INVALID_INPUT), 0);
    }

    #[test]
    fn example_2_valid_works() {
        assert_eq!(part2(PART2_VALID_INPUT), 4);
    }

    #[test]
    fn problem_2_works() {
        assert_eq!(part2(PROBLEM_INPUT), 184);
    }
}