use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Order {
    Ascending,
    Descending,
    Waiting,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    let mut total2 = 0;
    for line in contents.split("\n") {
        if line == "" {
            continue;
        }
        let mut list: Vec<i32> = Vec::new();
        for chars in line.split(" ") {
            list.push(chars.to_string().parse::<i32>().unwrap());
        }
        let part1 = is_safe(&list);
        if part1.0 {
            total += 1;
        } else {
            let bad_index = part1.1;
            list.remove(bad_index + 1);
            if is_safe(&list).0 {
                total2 += 1;
                println!("{:?} - resolved", list);
            }
        }
    }
    println!("part 1: {}", total);
    println!("part 2: {}", total + total2);
}

fn is_safe(list: &Vec<i32>) -> (bool, usize) {
    let mut list_order = Order::Waiting;
    for (i, current) in list.iter().enumerate() {
        let next = match list.get(i + 1) {
            Some(&n) => n,
            None => continue,
        };
        let difference = (current - next).abs();
        if difference > 3 || difference == 0 {
            println!("{:?} - bad difference", list);
            return (false, i);
        }
        let current_order = match &next > current {
            true => Order::Ascending,
            false => Order::Descending,
        };
        if list_order == Order::Waiting {
            list_order = current_order.clone();
            continue;
        }
        if list_order != current_order {
            println!("{:?} - bad order", list);
            return (false, i);
        }
    }
    (true, 0)
}
