use std::fs;

fn main() {
    let mut contents = fs::read_to_string("input.txt").unwrap();
    contents = contents.replace("\n", "");
    let total = muls(contents.as_str(), false);
    let total2 = muls(contents.as_str(), true);
    println!("part 1: {}", total);
    println!("part 2: {}", total2)
}

fn muls(line: &str, part2: bool) -> i32 {
    let mut total = 0;
    let mut operations: Vec<String> = Vec::new();
    let mut partial_operation = "".to_string();
    let dos = filter_op(line, "do()");
    let donts = filter_op(line, "don't()");
    let mut toggle = true;
    for (i, char) in line.chars().into_iter().enumerate() {
        if dos.contains(&i) {
            toggle = true;
            println!("on")
        }
        if donts.contains(&i) {
            toggle = false;
            println!("off")
        }
        partial_operation.push(char);
        if !is_valid_so_far(partial_operation.as_str()) {
            partial_operation.clear();
        } else {
            if char == ')' {
                if part2 && !toggle {
                    continue;
                }
                operations.push(partial_operation.clone());
                partial_operation.clear();
            }
        }
    }
    for op in operations {
        total += evaluate(op.as_str())
    }
    total
}

fn is_valid_so_far(partial_operation: &str) -> bool {
    let mut last_char = '¬';
    let mut checked = "".to_string();
    for char in partial_operation.chars() {
        if char.is_numeric() {
            if checked.starts_with("mul(") {
                checked.push(char);
                last_char = char;
                continue;
            } else {
                return false;
            }
        }
        match char.to_string().as_str() {
            "m" => {
                if last_char != '¬' {
                    return false;
                }
                last_char = char
            }
            "u" => {
                if last_char != 'm' {
                    return false;
                }
                last_char = char
            }
            "l" => {
                if last_char != 'u' {
                    return false;
                }
                last_char = char
            }
            "(" => {
                if last_char != 'l' {
                    return false;
                }
                last_char = char
            }
            "," | ")" => {
                if !last_char.is_numeric() {
                    return false;
                }
            }
            _ => return false,
        }
        checked.push(char);
    }
    true
}

fn evaluate(operation: &str) -> i32 {
    assert!(operation.starts_with("mul(") && operation.ends_with(")"));
    let mut op = operation.replace("mul(", "");
    op = op.replace(")", "");
    let pair = op.split_once(",").unwrap();
    if pair.0.len() > 3 || pair.1.len() > 3 {
        return 0;
    }
    let num1 = pair.0.parse::<i32>().unwrap();
    let num2 = pair.1.parse::<i32>().unwrap();
    num1 * num2
}

fn filter_op(line: &str, op: &str) -> Vec<usize> {
    let mut line = line.to_string();
    let mut indexes: Vec<usize> = Vec::new();
    while line.find(op) != Option::None {
        indexes.push(line.find(op).unwrap());
        line = line.replacen(op, "", 1);
    }
    assert!(!line.contains(op));
    indexes
}
