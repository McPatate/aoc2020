use std::fs;
use std::collections::HashMap;
use regex::Regex;

static FIELD_LIST: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn birth_year(byr: &str) -> bool {
    match byr.parse::<usize>() {
        Ok(r) => {
            if r >= 1920 && r <= 2002 {
                true
            }
            else {
                false
            }
        }
        Err(_) => false
    }
}

fn issue_year(iyr: &str) -> bool {
    match iyr.parse::<usize>() {
        Ok(r) => {
            if r >= 2010 && r <= 2020 {
                true
            }
            else {
                false
            }
        }
        Err(_) => false
    }
}

fn expiration_year(eyr: &str) -> bool {
    match eyr.parse::<usize>() {
        Ok(r) => {
            if r >= 2020 && r <= 2030 {
                true
            }
            else {
                false
            }
        }
        Err(_) => false
    }
}

fn height(hgt: &str) -> bool {
    if hgt.contains("cm") {
        let hgt_arr: Vec<&str> = hgt.split_terminator("cm").collect();
        return match hgt_arr[0].parse::<usize>() {
            Ok(h) => {
                if h >= 150 && h <= 193 {
                    true
                }
                else {
                    false
                }
            }
            Err(_) => false
        };
    }
    else if hgt.contains("in") {
        let hgt_arr: Vec<&str> = hgt.split_terminator("in").collect();
        return match hgt_arr[0].parse::<usize>() {
            Ok(h) => {
                if h >= 59 && h <= 76 {
                    true
                }
                else {
                    false
                }
            }
            Err(_) => false
        };
    }
    false
}

fn hair_color(hcl: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}").unwrap();
    re.is_match(hcl)
}

fn eye_color(ecl: &str) -> bool {
    let colors: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    colors.iter().any(|&f| f==ecl)
}

fn passport_id(pid: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(pid)
}

fn check_passports(passports: Vec<String>) -> usize {
    let mut array_of_fn: HashMap<&str, fn(&str)->bool> = HashMap::new();
    array_of_fn.insert("byr", birth_year);
    array_of_fn.insert("iyr", issue_year);
    array_of_fn.insert("eyr", expiration_year);
    array_of_fn.insert("hgt", height);
    array_of_fn.insert("hcl", hair_color);
    array_of_fn.insert("ecl", eye_color);
    array_of_fn.insert("pid", passport_id);

    let mut valid_ppts: usize = 0;
    for passport in passports {
        let fields: Vec<&str> = passport.split_terminator(' ').collect();
        let mut fields_nb = 0;
        if fields.len() > 6 {
            for field in fields {
                let kv: Vec<&str> = field.split(':').collect();
                let key = kv[0];
                if FIELD_LIST.iter().any(|&f| f==key) {
                    if array_of_fn[key](kv[1]) {
                        fields_nb += 1;
                    }
                }
            }
            if fields_nb == 7 {
                valid_ppts += 1;
            }
        }
    }
    valid_ppts
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    let mut passports: Vec<String> = Vec::new();
    let mut passport: String = String::new();
    for line in lines_as_strings {
        if line != "" {
            passport.push_str(line);
            passport.push(' ');
        }
        else {
            passports.push(passport);
            passport = String::new();
        }
    }
    passports.push(passport);
    println!("There are {} valid passports", check_passports(passports));
}
