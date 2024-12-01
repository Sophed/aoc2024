use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let parsed = parse(&contents);
    let mut left_list = parsed.0;
    let mut right_list = parsed.1;

    left_list.sort();
    right_list.sort();

    let mut total = 0;
    for (i, _) in left_list.iter().enumerate() {
        let left = left_list.get(i).unwrap();
        let right = right_list.get(i).unwrap();
        let distance = left.max(right) - right.min(left);
        total += distance;
    }
    println!("part 1: {}", total);
    total = 0;

    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for el in left_list {
        let count = match occurrences.get(&el) {
            Some(&n) => n,
            None => 0,
        };
        occurrences.insert(el, count + 1);
    }
    for el in right_list {
        let count = match occurrences.get(&el) {
            Some(&n) => n,
            None => 0,
        };
        total += el * count
    }
    println!("part 2: {}", total);
}

fn parse(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        let pair = line.split("   ").collect::<Vec<&str>>();
        let left = pair.get(0).unwrap().to_string().parse::<i32>().unwrap();
        let right = pair.get(1).unwrap().to_string().parse::<i32>().unwrap();
        left_list.push(left);
        right_list.push(right);
    }
    return (left_list, right_list);
}
