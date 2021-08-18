use std::collections::HashMap;

pub struct Group {
    pub answers: &'static str,
}

impl Group {
    pub fn new(answers: &'static str) -> Self {
        Self{ answers }
    }

    pub fn some_answered_yes(&self) -> HashMap<char, u32> {
        let mut map = HashMap::new();
        for q in self.answers.chars() {
            if q == '\n' {
                continue;
            }
            let counter = map.entry(q).or_insert(0);
            *counter += 1;
        }
        map
    }

    pub fn people(&self) -> usize {
        self.answers.split("\n").count()
    }

    pub fn all_answered_yes(&self) -> HashMap<char, u32> {
        let people = self.people();
        let mut map = self.some_answered_yes();
        map.retain(|_, v| *v == people as u32);
        map
    }
}