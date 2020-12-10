use std::fs;

struct Group {
    nb: usize,
    answers: String
}

impl Group {
    fn new(nb: usize, answers: String) -> Self {
        Group {
            nb,
            answers
        }
    }
}

fn sort_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn get_first_and_last_char(s: &str) -> (char, char) {
    let mut res: (char, char) = ('_', '_');
    let mut pos = 0;
    for c in s.chars() {
        if pos == 0 {
            res.0 = c;
        }
        else if pos == s.len() {
            res.1 = c;
        }
        pos += 1;
    }
    res
}

fn count_answers(groups: Vec<Group>) -> usize {
    let mut total_ans = 0;
    let mut prev_total = 0;
    for group in groups {
        let mut occ = 0;
        let (mut prev, last) = get_first_and_last_char(&group.answers);
        for c in group.answers.chars() {
            if c != prev || c == last {
                if occ == group.nb {
                    total_ans += 1;
                }
                occ = 1;
                prev = c;
            }
            else {
                occ += 1;
                prev = c;
            }
        }
        println!("{} people's answers, found {}. Input = {}", group.nb, total_ans - prev_total, &group.answers);
        prev_total = total_ans;
    }
    total_ans
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split('\n').collect();
    let mut groups: Vec<Group> = Vec::new();
    let mut answers: String = String::new();
    let mut nb_pers: usize = 0;
    for line in lines_as_strings {
        if line != "" {
            answers.push_str(line);
            nb_pers += 1;
        }
        else {
            groups.push(Group::new(nb_pers, sort_string(answers)));
            answers = String::new();
            nb_pers = 0;
        }
    }
    println!("There are {} answers", count_answers(groups));
}