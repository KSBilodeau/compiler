use std::io;
use std::io::Write;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Sub),
            '*' => Ok(Operation::Mul),
            '/' => Ok(Operation::Div),
            _ => Err(format!("{} IS AN UNDEFINED OPERATION", value))
        }
    }
}

#[derive(Debug)]
struct Term<'a> {
    content: &'a str,
    _span: std::ops::Range<usize>,
}

#[derive(Debug)]
pub struct Input<'a> {
    term_ranges: Vec<Term<'a>>,
    operations: Vec<Operation>,
}

pub fn demarcate(input: &str) -> Input {
    let mut char_stream = input.chars().enumerate().peekable();

    let mut term_ranges = Vec::new();
    let mut operations = Vec::new();

    while let Some((start_idx, char)) = char_stream.next() {
        if char.is_numeric() {
            let mut offset = 0usize;
            while char_stream.peek().is_some_and(|x| x.1.is_numeric()) {
                char_stream.next();
                offset += 1;
            }

            term_ranges.push(Term {
                content: &input[start_idx..start_idx + offset + 1],
                _span: start_idx..(start_idx + offset + 1)
            });
        } else if let Ok(op) = Operation::try_from(char) {
            operations.push(op);
        }
    }

    Input {
        term_ranges,
        operations,
    }
}

pub fn parse(input: &str) -> isize {
    let demarcated_input = demarcate(input);

    if demarcated_input.term_ranges.len() == 0 {
        println!("NO VALID INPUT PROVIDED");
        return isize::MIN;
    }

    let mut term_stream = demarcated_input.term_ranges.iter().peekable();
    let mut op_stream = demarcated_input.operations.iter();

    let term = term_stream.next().unwrap().content;
    let mut result = term.parse::<isize>().unwrap();

    while let Some(&ref term) = term_stream.next() {
        let term = term.content.parse::<isize>().unwrap();
        let operation = op_stream.next().unwrap();

        match operation {
            Operation::Add => result += term,
            Operation::Sub => result -= term,
            Operation::Mul => result *= term,
            Operation::Div => result /= term,
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
