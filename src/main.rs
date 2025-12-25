#[derive(Debug)]
enum Type {
    Expression(Box<Expression>),
    Number(i32),
    Operator(char),
}
#[derive(Debug)]
struct Expression {
    left: Type,
    right: Type,
    operator: char,
}

fn tokenize(input: String) -> Vec<String> {
    let mut num_buffer = String::new();
    let mut tokens: Vec<String> = Vec::new();

    for char in input.chars() {
        if char.is_ascii_digit() {
            num_buffer.push(char);
        } else if char.is_whitespace() {
            if !num_buffer.is_empty() {
                tokens.push(num_buffer.clone());
                num_buffer.clear();
            }
        } else if "+*/-".contains(char) {
            if !num_buffer.is_empty() {
                tokens.push(num_buffer.clone());
                num_buffer.clear();
            }
            tokens.push(char.to_string());
        }
    }
    if !num_buffer.is_empty() {
        tokens.push(num_buffer.clone());
        num_buffer.clear();
    }
    tokens
}

fn group_expression(input: Vec<String>) -> Vec<Type> {
    let mut grouped: Vec<Type> = Vec::new();
    let mut count = 0;
    while count < input.len() {
        let operator = input[count + 1].chars().next().unwrap();
        match input.get(count + 1) {
            Some(op) if op == "*" || op == "/" => {
                grouped.push(Type::Expression(Box::new(Expression {
                    left: Type::Number(input[count].parse::<i32>().unwrap()),
                    operator: operator,
                    right: Type::Number(input[count + 2].parse::<i32>().unwrap()),
                })));
                count += 3;
                continue;
            }
            Some(op) if op == "+" || op == "-" => {
                grouped.push(Type::Expression(Box::new(Expression {
                    left: Type::Number(input[count].parse::<i32>().unwrap()),
                    operator: operator,
                    right: Type::Number(input[count + 2].parse::<i32>().unwrap()),
                })));
                count += 3;
                continue;
            }
            _ => {
                if "*-+/".contains(input[count].chars().next().unwrap()) {
                    grouped.push(Type::Operator(input[count].chars().next().unwrap()));
                } else {
                    grouped.push(Type::Number(input[count].parse::<i32>().unwrap()));
                }
            }
        }

        count += 1;
    }
    grouped
}

fn evaluate_expression(input: &Expression) -> i32 {
    let left = match &input.left {
        Type::Expression(e) => evaluate_expression(e),
        Type::Number(n) => *n,
        _ => panic!("Something ain't right chief"),
    };
    let right = match &input.right {
        Type::Expression(e) => evaluate_expression(e),
        Type::Number(n) => *n,
        _ => panic!("Something ain't right chief"),
    };
    match input.operator {
        '*' => left * right,
        '/' => left / right,
        '-' => left - right,
        '+' => left + right,
        _ => panic!("Unknown Operator"),
    }
}

fn evaluate_all(input: Vec<Type>) -> i32 {
    let mut result = match &input[0] {
        Type::Expression(e) => evaluate_expression(e),
        Type::Number(n) => *n,
        _ => panic!("Cannot Start With Operator!"),
    };
    let mut count = 1;
    while count < input.len() {
        let op = match &input[count] {
            Type::Operator(op) => *op,
            _ => panic!("Must be an Operator!"),
        };
        let right = match &input[count + 1] {
            Type::Expression(e) => evaluate_expression(e),
            Type::Number(n) => *n,
            _ => panic!("Unexpected type shii"),
        };
        match op {
            '*' => result *= right,
            '-' => result -= right,
            '+' => result += right,
            '/' => result /= right,
            _ => panic!("Unexpected Operator"),
        }
        count += 2
    }
    result
}
fn main() {
    let tokens = tokenize(String::from("2-5"));
    println!("{:?}", evaluate_all(group_expression(tokens)));
}
