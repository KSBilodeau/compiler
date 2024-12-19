use std::io;
use std::io::Write;
use lyn::Scanner;

pub enum Calc {
    Expr {
        lhs: usize,
        op: Operation,
        rhs: Box<Calc>,
    },
    Number(usize)
}

pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
}

pub fn expr(state: &mut Scanner) -> Calc {
    let term = number(state);

    if state.is_done() {
        Calc::Number(term)
    } else {
        Calc::Expr {
            lhs: term,
            op: operation(state),
            rhs: Box::new(expr(state)),
        }
    }
}

pub fn number(state: &mut Scanner) -> usize {
    let mut num_str = String::new();

    loop {
        num_str.push(*state.pop().unwrap());

        if state.is_done() || !state.peek().unwrap().is_numeric() {
            break;
        }
    }

    num_str.parse().unwrap()
}

pub fn operation(state: &mut Scanner) -> Operation {
    state.transform(|char| match char {
        '+' => Some(Operation::Add),
        '-' => Some(Operation::Sub),
        '*' => Some(Operation::Mul),
        '/' => Some(Operation::Div),
        _ => None,
    }).unwrap()
}

pub fn parse(input: &str) -> usize {
    let mut state = Scanner::new(input.trim());

    let mut result;
    let mut next_op;
    let mut next_expr = expr(&mut state);

    match next_expr {
        Calc::Expr { lhs, rhs, op } => {
            result = lhs;
            next_op = op;
            next_expr = *rhs;
        }
        Calc::Number(num) => return num,
    }

    loop {
        match next_expr {
            Calc::Expr { lhs, rhs, op} => {
                match next_op {
                    Operation::Add => result += lhs,
                    Operation::Sub => result -= lhs,
                    Operation::Div => result /= lhs,
                    Operation::Mul => result *= lhs,
                }

                next_op = op;
                next_expr = *rhs;
            }
            Calc::Number(num) => {
                match next_op {
                    Operation::Add => result += num,
                    Operation::Sub => result -= num,
                    Operation::Div => result /= num,
                    Operation::Mul => result *= num,
                }

                break
            }
        }
    }

    result
}

fn main() {

    loop {
        print!("Please enter a string: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("{}", parse(&input));
    }
}
