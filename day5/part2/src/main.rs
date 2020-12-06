use std::fs;

fn convert_to_nb(one: char, input: &str) -> usize {
    let mut binary: Vec<bool> = Vec::with_capacity(input.len());
    for c in input.chars() {
        if c == one {
            binary.push(true);
        }
        else {
            binary.push(false);
        }
    }
    let mut nb = 0;
    let mut i = 1;
    let input_len = binary.len();
    for v in binary {
        if v {
            nb += 2usize.pow((input_len - i) as u32);
        }
        i += 1;
    }
    nb
}

fn calc_id(bp: &str) -> usize {
    let row = convert_to_nb('B', &bp[..7]);
    let col = convert_to_nb('R', &bp[7..]);
    row * 8 + col
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    let mut ids: Vec<usize> = Vec::with_capacity(lines_as_strings.len());
    for line in lines_as_strings {
        ids.push(calc_id(line))
    }
    ids.sort();
    let mut current = 0;
    let mut prev = 0;
    for id in ids {
        current = id;
        if current == prev + 2 {
            break;
        }
        prev = current;
    }
    println!("My seat ID = {}", current - 1);
}
