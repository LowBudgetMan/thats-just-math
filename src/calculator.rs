fn calculate(expression: &str) -> i8 {
    return if expression.contains('+') {
        let parts: Vec<&str> = expression.split('+').collect();
        parts[0].parse::<i8>().unwrap() + parts[1].parse::<i8>().unwrap()
    } else {
        let parts: Vec<&str> = expression.split('-').collect();
        parts[0].parse::<i8>().unwrap() - parts[1].parse::<i8>().unwrap()
    }
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