use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines_as_strings {
        map.push(line.chars().collect());
    }
    map.remove(0);
    let mut trees: usize = 0;
    let mut x: usize = 3;
    for line in map {
        if line[x] == '#' {
            trees += 1;
        }
        x += 3;
        if x > line.len() - 1 {
            x = x - line.len()
        }

    }
    println!("There are {} trees", trees);
}
