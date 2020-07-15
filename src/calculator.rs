pub fn calculate(expression: &str) -> i8 {
    return if expression.contains('/') {
        let parts = split_by_operand(expression, '/');
        calculate(parts[0]) / calculate(parts[1])
    }
    else if expression.contains('*') {
        let parts = split_by_operand(expression, '*');
        calculate(parts[0]) * calculate(parts[1])
    }
    else if expression.contains('+') {
        let parts = split_by_operand(expression, '+');
        calculate(parts[0]) + calculate(parts[1])
    }
    else if expression.contains('-') {
        let parts = split_by_operand(expression, '-');
        calculate(parts[0]) - calculate(parts[1])
    }
    else {
        parse_number(expression)
    }
}

fn split_by_operand(expression: &str, operand: char) -> Vec<&str> {
    return expression.splitn(2, operand).collect();
}

fn parse_number(expression: &str) -> i8 {
    return expression.parse::<i8>().unwrap();
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

#[test]
fn can_divide() {
    let result = calculate("2/2");
    assert_eq!(result, 1);
}

#[test]
fn can_handle_complex_statement() {
    let result = calculate("2+2-1");
    assert_eq!(result, 3);
}
