use std::fs;

fn count_trees(map: &Vec<Vec<char>>, right: usize, step: usize) -> usize {
    let mut trees: usize = 0;
    let mut x: usize = right;
    for line in map.iter().skip(step).step_by(step) {
        if line[x] == '#' {
            trees += 1;
        }
        x += right;
        if x > line.len() - 1 {
            x = x - line.len()
        }
    }
    trees
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    let rules = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees: usize = 1;
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines_as_strings {
        map.push(line.chars().collect());
    }
    for rule in rules {
        trees *= count_trees(&map, rule.0, rule.1);
    }
    println!("Multiply of all tree nbs = {}", trees);
}
