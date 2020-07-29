pub fn calculate(expression: &str) -> i128 {
    let modified_expression: String = replace_negative_number_notation(expression);
    return recursive_calculate(&modified_expression)
}

fn recursive_calculate(modified_expression: &str) -> i128 {
    return if modified_expression.contains('(') {
        let simplified_expression = calculate_parenthetic_expression(modified_expression);
        calculate(&simplified_expression)
    } else if modified_expression.contains('+') {
        let parts = split_by_operand(modified_expression, '+');
        calculate(parts[0]) + calculate(parts[1])
    } else if modified_expression.contains('-') {
        let parts = split_by_operand(modified_expression, '-');
        calculate(parts[0]) - calculate(parts[1])
    } else if modified_expression.contains('/') {
        let parts = split_by_operand(modified_expression, '/');
        calculate(parts[0]) / calculate(parts[1])
    } else if modified_expression.contains('*') {
        let parts = split_by_operand(modified_expression, '*');
        calculate(parts[0]) * calculate(parts[1])
    } else {
        parse_number(modified_expression)
    }
}

fn replace_negative_number_notation(expression: &str) -> String{
    let mut modified_expression = String::with_capacity(expression.len());
    for (index, found_char) in expression.chars().enumerate() {
        if found_char == '-' {
            if index == 0 || char_contains_symbol(expression.chars().nth(index-1).unwrap()) {
                modified_expression.push('`')
            }
            else {
                modified_expression.push('-')
            }
        } else {
            modified_expression.push(found_char)
        }
    }
    return modified_expression
}

fn char_contains_symbol(value: char) -> bool {
    return "-+*/".to_string().contains(value)
}

fn calculate_parenthetic_expression(expression: &str) -> String {
    let left_parenthesis_index = expression.to_string().find('(').unwrap();
    let right_parenthesis_index = expression.to_string().rfind(')').unwrap();
    let left_of_expression = &expression[..left_parenthesis_index];
    let middle_of_expression = calculate(&expression[left_parenthesis_index + 1..right_parenthesis_index]);
    let right_of_expression = &expression[right_parenthesis_index + 1..];
    let simplified_expression = format!("{}{}{}", &left_of_expression, middle_of_expression, &right_of_expression);
    return simplified_expression
}

fn split_by_operand(expression: &str, operand: char) -> Vec<&str> {
    return expression.splitn(2, operand).collect();
}

fn parse_number(expression: &str) -> i128 {
    return expression.to_string().replace('`', "-").parse::<i128>().unwrap();
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
fn can_handle_complex_addition_subtraction_statement() {
    let result = calculate("2+2-1");
    assert_eq!(result, 3);
}

#[test]
fn associative_property_of_addition() {
    let result = calculate("2-1+2");
    assert_eq!(result, 3);
}

#[test]
fn can_handle_complex_multiplication_division_statement() {
    let result = calculate("2*3/2");
    assert_eq!(result, 3);
}

#[test]
fn can_handle_multiplication_division_addition_subtraction_order_of_operations() {
    let result = calculate("2+7*3-1");
    assert_eq!(result, 22);
}

#[test]
fn can_handle_large_numbers() {
    let result = calculate("17014118346046923173168730371588410572");
    assert_eq!(result, 17014118346046923173168730371588410572);
}

#[test]
fn can_handle_negative_numbers() {
    let result = calculate("-3--2");
    assert_eq!(result, -1);
}

#[test]
fn can_handle_negative_numbers_mixed_with_positive_operands() {
    let result = calculate("-3+-2");
    assert_eq!(result, -5);
}

#[test]
fn can_handle_negative_number_on_its_own() {
    let result = calculate("-128");
    assert_eq!(result, -128);
}

#[test]
fn can_handle_parenthesis_at_beginning() {
    let result = calculate("(2+7)*2");
    assert_eq!(result, 18);
}

#[test]
fn can_handle_parenthesis_in_middle() {
    let result = calculate("4*(2+1)*2");
    assert_eq!(result, 24);
}

#[test]
fn can_handle_parenthesis_at_end() {
    let result = calculate("4*(2+3)");
    assert_eq!(result, 20);
}

#[test]
fn can_handle_nested_parenthesis() {
    let result = calculate("4*(3+(4/2))");
    assert_eq!(result, 20);
}

#[test]
fn cboyer17_example() {
    let result = calculate("1+(3*(1+2))-8");
    assert_eq!(result, 2);
}
