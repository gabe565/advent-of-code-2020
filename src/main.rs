fn main() {
    let day = std::env::args()
        .nth(1).expect("no day given")
        .parse().expect("invalid number");

    let func = match day {
        _ => panic!("invalid day"),
    };

    func();
}