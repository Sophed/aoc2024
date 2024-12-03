use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total = 0;
    for line in contents.split("\n") {
        total += muls(line)
    }
    println!("part 1: {}", total)
}

fn muls(line: &str) -> i32 {
    let mut total = 0;
    let mut operations: Vec<String> = Vec::new();
    let mut partial_operation = "".to_string();
    for char in line.chars() {
        partial_operation.push(char);
        if !is_valid_so_far(partial_operation.as_str()) {
            partial_operation.clear();
        } else {
            if char == ')' {
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
