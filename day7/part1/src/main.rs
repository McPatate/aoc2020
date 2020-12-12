use std::fs;
use std::collections::HashMap;

fn find_all_sb(bags: Vec<&str>, contents: HashMap<&str, Vec<String>>) -> usize {
    let mut total = 0;
    for bag in bags {
        if bag != "shiny gold bag" {
            let search = contents.get(&bag).unwrap();
            while search.iter().any(|&v| v == "no other bags.") {
                if  == "shiny gold bag" {
                    total += 1;
                    break;
                }
            }
        }
    }
    total
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let lines_as_strings: Vec<&str> = input.split_terminator('\n').collect();
    let mut bags: Vec<&str> = Vec::new();
    let mut contents: HashMap<&str, Vec<String>> = HashMap::new();
    for line in lines_as_strings {
        let elements: Vec<&str> = line.split(" contain ").collect();
        bags.push(elements[0]);
        let mut sub_bags: Vec<String> = Vec::new();
        let sub_bags_raw = elements[1].split(", ").collect::<Vec<&str>>();
        for bag in sub_bags_raw {
            let el = bag.split(' ').collect::<Vec<&str>>();
            let nb = el[0].parse::<usize>();
            match nb {
                Ok(_) => sub_bags.push(el[1..].join(" ").replace("bags", "bag").replace('.', "")),
                Err(_) => sub_bags.push(el.join(" "))
            }
        }
        contents.insert(elements[0], sub_bags);
    }
    println!("{} Shiny Gold bags", find_all_sb(bags, contents));
}
