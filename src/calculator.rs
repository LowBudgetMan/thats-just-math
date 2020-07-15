fn calculate(expression: &str) -> i8 {
    let parts: Vec<&str> = expression.split('+').collect();
    return parts[0].parse::<i8>().unwrap() + parts[1].parse::<i8>().unwrap();
}

#[test]
fn can_add() {
    let result = calculate("1+1");
    assert_eq!(result, 2);
}