use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Seat {
    pub id: i32,
}

impl Seat {
    pub fn new(id: i32) -> Self{
        Self{ id }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "seat ID {}", self.id)
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}