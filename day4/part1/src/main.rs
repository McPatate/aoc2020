use std::fs;

static FIELD_LIST: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn check_passports(passports: Vec<String>) -> usize {
    let mut valid_ppts: usize = 0;
    for passport in passports {
        let fields: Vec<&str> = passport.split_terminator(' ').collect();
        let mut fields_nb = 0;
        if fields.len() > 6 {
            for field in fields {
                let kv: Vec<&str> = field.split(':').collect();
                let key = kv[0];
                if FIELD_LIST.iter().any(|&f| f==key) {
                    fields_nb += 1;
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
