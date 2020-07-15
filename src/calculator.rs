fn calculate(expression: &str) -> i8 {
    return if expression.contains('+') {
        let parts = split_by_operand(expression, '+');
        parts.0 + parts.1
    } else {
        let parts = split_by_operand(expression, '-');
        parts.0 - parts.1
    }
}

fn split_by_operand(expression: &str, operand: char) -> (i8, i8) {
    let parts: Vec<&str> = expression.split(operand).collect();
    return (parts[0].parse::<i8>().unwrap(), parts[1].parse::<i8>().unwrap());
}

#[test]
fn can_add() {
    let result = calculate("1+1");
    assert_eq!(result, 2);
}

#[test]
fn can_subtract() {
    let result = calculate("1-1");
    assert_eq!(result, 0);
}

#[test]
fn can_multiply() {
    let result = calculate("2*2");
    assert_eq!(result, 4);
}
