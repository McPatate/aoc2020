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

fn count_answers(groups: Vec<Group>) -> usize {
    let mut total_ans = 0;
    for group in groups {
        let mut deduped: Vec<char> = group.answers.chars().collect();
        deduped.dedup();
        for c in  deduped {
            if group.nb == group.answers.matches(c).count() {
                total_ans += 1;
            }
        }
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