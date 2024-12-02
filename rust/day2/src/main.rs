use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        let mut list: Vec<i32> = Vec::new();
        for chars in line.split(" ") {
            list.push(chars.to_string().parse::<i32>().unwrap());
        }
        if is_safe(list) {
            total += 1;
        }
    }
    println!("part 1: {}", total);
}

fn is_safe(list: Vec<i32>) -> bool {
    let mut skip = false;
    let mut count: i32 = 0;
    for (i, current) in list.iter().enumerate() {
        let next = match list.get(i + 1) {
            Some(&n) => n,
            None => continue,
        };
        let difference = (current - next).abs();
        if difference > 3 || difference == 0 {
            skip = true;
            break;
        }
        if &next > current {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if skip {
        return false;
    }
    if count.abs() == (list.len() as i32) - 1 {
        return true;
    }
    false
}
