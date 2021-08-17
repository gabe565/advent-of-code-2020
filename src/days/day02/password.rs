use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<target>.): (?P<pass>.*)$").unwrap();
}

pub struct Password {
    pub first: u32,
    pub second: u32,
    pub target: char,
    pub password: &'static str,
}

impl Password {
    pub fn new(first: u32, second: u32, target: char, password: &'static str) -> Self {
        Self {
            first,
            second,
            target,
            password,
        }
    }

    pub fn part1_valid(&self) -> bool {
        let count = self.password.matches(self.target).count() as u32;
        self.first <= count && count <= self.second
    }

    pub fn part2_valid(&self) -> bool {
        let mut result = false;
        let chars = self.password.chars().collect::<Vec<char>>();
        for i in [self.first - 1, self.second - 1] {
            if chars[i as usize] == self.target {
                result = !result;
            }
        }
        result
    }
}

impl From<&'static str> for Password {
    fn from(s: &'static str) -> Self {
        let cap = RE.captures(s).unwrap();
        Self::new(
            cap.name("min").unwrap().as_str().parse().unwrap(),
            cap.name("max").unwrap().as_str().parse().unwrap(),
            cap.name("target").unwrap().as_str().parse().unwrap(),
            cap.name("pass").unwrap().as_str(),
        )
    }
}