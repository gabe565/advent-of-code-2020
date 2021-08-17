pub struct Passport {
    byr: Option<&'static str>,
    iyr: Option<&'static str>,
    eyr: Option<&'static str>,
    hgt: Option<&'static str>,
    hcl: Option<&'static str>,
    ecl: Option<&'static str>,
    pid: Option<&'static str>,
    cid: Option<&'static str>,
}

#[allow(dead_code)]
impl Passport {
    pub fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn from_str(input: &'static str) -> Self {
        let mut passport = Passport::new();
        let kv = input.split_whitespace().map(
            |x| x.split_once(":").unwrap()
        );
        for (k, v) in kv {
            passport.set(k, v);
        }
        passport
    }

    pub fn set(&mut self, k: &'static str, v: &'static str) {
        match k {
            "byr" => self.byr = Some(v),
            "iyr" => self.iyr = Some(v),
            "eyr" => self.eyr = Some(v),
            "hgt" => self.hgt = Some(v),
            "hcl" => self.hcl = Some(v),
            "ecl" => self.ecl = Some(v),
            "pid" => self.pid = Some(v),
            "cid" => self.cid = Some(v),
            _ => panic!("invalid value {}", k),
        }
    }

    pub fn part1_is_valid(&self) -> bool {
        !self.byr.is_none() && !self.iyr.is_none() && !self.eyr.is_none() &&
            !self.hgt.is_none() && !self.hcl.is_none() && !self.ecl.is_none() &&
            !self.pid.is_none()
    }

    pub fn byr_is_valid(&self) -> bool {
        match self.byr {
            Some(i) =>
                match i.parse::<u32>() {
                    Ok(i) => 1920 <= i && i <= 2002,
                    Err(_e) => false,
                },
            None => false,
        }
    }

    pub fn iyr_is_valid(&self) -> bool {
        match self.iyr {
            Some(i) =>
                match i.parse::<u32>() {
                    Ok(i) => 2010 <= i && i <= 2020,
                    Err(_e) => false,
                },
            None => false,
        }
    }

    pub fn eyr_is_valid(&self) -> bool {
        match self.eyr {
            Some(i) =>
                match i.parse::<u32>() {
                    Ok(i) => 2020 <= i && i <= 2030,
                    Err(_e) => false,
                },
            None => false,
        }
    }

    pub fn hgt_is_valid(&self) -> bool {
        match self.hgt {
            Some(i) => {
                let (str, unit) = i.split_at(i.len() - 2);
                let hgt = str.parse::<u32>();
                match hgt {
                    Ok(i) =>
                        match unit {
                            "cm" => 150 <= i && i <= 193,
                            "in" => 59 <= i && i <= 76,
                            _ => false,
                        },
                    Err(_e) => false,
                }
            }
            None => false,
        }
    }

    pub fn hcl_is_valid(&self) -> bool {
        match self.hcl {
            Some(i) => i.starts_with('#') && i.len() == 7
                && i.chars().any(|c| matches!(c, 'a'..='f' | '0'..='9')),
            None => false,
        }
    }

    pub fn ecl_is_valid(&self) -> bool {
        match self.ecl {
            Some(i) =>
                match i {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                },
            None => false,
        }
    }

    pub fn pid_is_valid(&self) -> bool {
        match self.pid {
            Some(i) => i.len() == 9 && i.parse::<u32>().is_ok(),
            None => false,
        }
    }

    pub fn cid_is_valid(&self) -> bool {
        true
    }

    pub fn part2_is_valid(&self) -> bool {
        self.byr_is_valid() && self.iyr_is_valid() && self.eyr_is_valid()
            && self.hgt_is_valid() && self.hcl_is_valid() && self.ecl_is_valid()
            && self.pid_is_valid() && self.cid_is_valid()
    }
}