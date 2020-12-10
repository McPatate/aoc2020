use std::fs;

fn count_all_ans(answers: Vec<String>) -> usize {
    let mut total = 0;
    for ans in answers {
        if ans != "" {
            total += ans.len();
        }
    }
    total
}

fn merge_str(left: &str, right: &str) -> String {
    let mut res = String::new();
    for c in left.chars() {
        if right.contains(c) {
            res.push(c);
        }
    }
    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split('\n').collect();
    let mut groups: Vec<String> = Vec::new();
    let mut answers: String = String::new();
    for line in lines_as_strings {
        if line != "" {
            if !answers.is_empty() {
                answers = merge_str(&answers, line);
            }
            else {
                answers.push_str(line);
            }
        }
        else {
            groups.push(answers);
            answers = String::new();
        }
    }
    println!("{:?}", &groups);
    println!("There are {} yes'", count_all_ans(groups));
}
