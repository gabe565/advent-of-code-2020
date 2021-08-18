const EXAMPLE_INPUT: &'static str = include_str!("example.txt");
const PROBLEM_INPUT: &'static str = include_str!("problem.txt");
const SUM: u32 = 2020;

fn parse_input(input: &'static str) -> Vec<u32> {
    input.split("\n")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
}

fn part1(input: &'static str) -> Option<u32> {
    let list = parse_input(input);

    for x in &list {
        for y in &list {
            if x == y {
                continue;
            }
            if x + y == SUM {
                return Some(x * y);
            }
        }
    }
    None
}

fn part2(input: &'static str) -> Option<u32> {
    let list = parse_input(input);

    for x in &list {
        for y in &list {
            if x == y || x + y > SUM {
                continue;
            }
            for z in &list {
                if z == x || z == y {
                    continue;
                }
                if x + y + z == SUM {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}

pub fn main() {
    println!("1.1 Example: {}", part1(EXAMPLE_INPUT).unwrap());
    println!("1.1 Problem: {}", part1(PROBLEM_INPUT).unwrap());
    println!("1.2 Example: {}", part2(EXAMPLE_INPUT).unwrap());
    println!("1.2 Problem: {}", part2(PROBLEM_INPUT).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_works() {
        assert_eq!(part1(EXAMPLE_INPUT).unwrap(), 514579);
    }

    #[test]
    fn problem_1_works() {
        assert_eq!(part1(PROBLEM_INPUT).unwrap(), 326211);
    }

    #[test]
    fn example_2_works() {
        assert_eq!(part2(EXAMPLE_INPUT).unwrap(), 241861950);
    }

    #[test]
    fn problem_2_works() {
        assert_eq!(part2(PROBLEM_INPUT).unwrap(), 131347190);
    }
}