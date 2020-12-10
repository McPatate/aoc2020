use std::fs;

fn count_ans(ans: &str) -> usize {
    let mut chars: Vec<char> = ans.chars().collect();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}

fn count_all_ans(answers: Vec<String>) -> usize {
    let mut total = 0;
    for ans in answers {
        total += count_ans(&ans);
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split('\n').collect();
    let mut groups: Vec<String> = Vec::new();
    let mut answers: String = String::new();
    for line in lines_as_strings {
        if line != "" {
            answers.push_str(line);
        }
        else {
            groups.push(answers);
            answers = String::new();
        }
    }
    println!("There are {} yes'", count_all_ans(groups));
}
