use std::io;

/*
For part 2:

with help from http://math.hws.edu/javanotes/c9/s5.html

expression =>
| term
| term multiply term

term =>
| part
| part add part

part =>
| number
| leftparen expression rightparen
*/

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(i64),
    Add,
    Multiply,
    LeftParen,
    RightParen,
}

fn lex(string: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for token in string.chars() {
        if token == ' ' {
            continue;
        }

        tokens.push(match token {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Token::Number(token.to_string().parse::<i64>().unwrap())
            }
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' => Token::Add,
            '*' => Token::Multiply,
            _ => panic!("found unknown token {}", token),
        })
    }

    tokens
}

fn reduce(stack: &mut Vec<Token>) {
    loop {
        if stack.len() < 3 {
            break;
        }

        match stack.last() {
            Some(Token::Add) | Some(Token::Multiply) => break,
            _ => (),
        }

        // look at the last three
        let third = stack[stack.len() - 1];
        let second = stack[stack.len() - 2];
        let first = stack[stack.len() - 3];

        match (first, second, third) {
            (Token::LeftParen, Token::Number(_), Token::RightParen) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(second);
            }
            (Token::Number(a), Token::Add, Token::Number(b)) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Number(a + b));
            }
            (Token::Number(a), Token::Multiply, Token::Number(b)) => {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.push(Token::Number(a * b));
            }
            _ => break,
        }
    }
}

fn eval_expression(string: String) -> i64 {
    let tokens = lex(&string);

    let mut stack = Vec::new();

    for token in tokens {
        stack.push(token);
        reduce(&mut stack);
    }

    assert_eq!(stack.len(), 1);

    match stack.pop().unwrap() {
        Token::Number(result) => result,
        _ => panic!("result is not number"),
    }
}

fn get_part(tokens: &Vec<Token>, pos: usize) -> (i64, usize) {
    match tokens[pos] {
        Token::Number(n) => (n, pos + 1),
        Token::LeftParen => {
            let (n, next_pos) = get_expression(tokens, pos + 1);
            match tokens[next_pos] {
                Token::RightParen => (n, next_pos + 1),
                _ => panic!("rightparen expected at {}", next_pos),
            }
        }
        _ => panic!("unexpected token {:?} at {}", tokens[pos], pos),
    }
}

fn get_term(tokens: &Vec<Token>, pos: usize) -> (i64, usize) {
    let (mut value, mut next_pos) = get_part(tokens, pos);

    while next_pos < tokens.len() {
        match tokens[next_pos] {
            Token::Add => {
                let (other_value, pos) = get_part(tokens, next_pos + 1);
                value += other_value;
                next_pos = pos;
            }
            _ => break,
        }
    }
    (value, next_pos)
}

fn get_expression(tokens: &Vec<Token>, pos: usize) -> (i64, usize) {
    let (mut value, mut next_pos) = get_term(tokens, pos);

    while next_pos < tokens.len() {
        match tokens[next_pos] {
            Token::Multiply => {
                let (other_term, pos) = get_term(tokens, next_pos + 1);
                value *= other_term;
                next_pos = pos;
            }
            _ => break,
        }
    }
    (value, next_pos)
}

fn rec_dec_parser(tokens: &Vec<Token>) -> i64 {
    let (result, pos) = get_expression(tokens, 0);
    if pos < tokens.len() {
        panic!("syntax error!");
    }
    result
}

/// evaluate expression using recursive descent (grammar in comment at top)
fn eval_expression2(string: String) -> i64 {
    let tokens = lex(&string);
    rec_dec_parser(&tokens)
}

pub fn day18(part_a: bool) {
    let mut sum = 0;

    let eval_fn = if part_a {
        eval_expression
    } else {
        eval_expression2
    };

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                println!("{}", line.trim());
                let result = eval_fn(line.trim().to_string());
                println!("Evaluated as: {}", result);
                sum += result;
            }
        };
    }

    println!("answer: {}", sum);
}
