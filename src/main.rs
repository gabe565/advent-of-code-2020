mod days;

fn main() {
    let day = std::env::args()
        .nth(1).expect("no day given")
        .parse().expect("invalid number");

    let func = match day {
        1 => days::day01::main,
        2 => days::day02::main,
        3 => days::day03::main,
        4 => days::day04::main,
        5 => days::day05::main,
        _ => panic!("invalid day"),
    };

    func();
}