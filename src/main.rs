use std::{ collections::HashMap, io::{ self, Write } };

fn main() {
    println!("Math Calculator - Type 'exit' to quit");

    loop {
        print!("Enter Expression: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        match calculate(input) {
            Ok(res) => println!("Result: {}", res),
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    let tokens = gen_token_from_input(input)?;
    let rpn = gen_rpn(&tokens)?;
    evaluate_rpn(&rpn)
}

fn gen_token_from_input(input: &str) -> Result<Vec<String>, String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut num = String::new();

    for val in input.chars() {
        if val.is_digit(10) || val == '.' {
            num.push(val);
        } else if "+-*/()".contains(val) {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push(val.to_string());
        } else if !val.is_whitespace() {
            return Err(format!("Invalid character: {}", val));
        }
    }

    if !num.is_empty() {
        tokens.push(num);
    }

    Ok(tokens)
}

fn gen_rpn(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let precedence: HashMap<&str, i32> = [
        ("+", 1),
        ("-", 1),
        ("*", 2),
        ("/", 2),
    ]
        .iter()
        .cloned()
        .collect();

    for token in tokens {
        if let Ok(_) = token.parse::<f64>() {
            output.push(token.clone());
        } else if precedence.contains_key(token.as_str()) {
            while let Some(op) = operators.last() {
                if
                    precedence.get(op.as_str()).unwrap_or(&0) >=
                    precedence.get(token.as_str()).unwrap()
                {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token.clone());
        } else if token == "(" {
            operators.push(token.clone());
        } else if token == ")" {
            while let Some(op) = operators.pop() {
                if op == "(" {
                    break;
                } else {
                    output.push(op);
                }
            }
        } else {
            return Err(format!("Invalid token: {}", token));
        }
    }

    while let Some(op) = operators.pop() {
        if op == "(" {
            return Err("Mismatched parentheses".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(rpn: &[String]) -> Result<f64, String> {
    let mut num_values: Vec<f64> = Vec::new();

    for val in rpn {
        if let Ok(i) = val.parse::<f64>() {
            num_values.push(i);
        } else if "+-*/".contains(val) {
            if num_values.len() < 2 {
                return Err("Invalid RPN Expression".to_string());
            }
            let b = num_values.pop().unwrap();
            let a = num_values.pop().unwrap();

            let cal_value = match val.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    a / b
                }
                _ => unreachable!(),
            };

            num_values.push(cal_value);
        } else {
            return Err("Unknown Expression".to_string());
        }
    }

    if num_values.len() != 1 {
        return Err("Invalid Expression".to_string());
    }

    Ok(num_values.pop().unwrap())
}
