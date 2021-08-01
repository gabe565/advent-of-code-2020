mod days;

fn main() {
    let day = std::env::args()
        .nth(1).expect("no day given")
        .parse().expect("invalid number");

    let func = match day {
        1 => days::day01::main,
        _ => panic!("invalid day"),
    };

    func();
}