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
    let mut lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    lines_as_strings.sort();
    let mut pos = 0;
    let mut prev_row = &lines_as_strings[0][..7];
    for line in lines_as_strings.iter().skip(1) {
        let row = &line[..7];
        if row != prev_row {
            break;
        }
        prev_row = row;
        pos += 1;
    }
    println!("Highest seat ID = {} ({})", calc_id(&lines_as_strings[pos]), lines_as_strings[pos]);
}
