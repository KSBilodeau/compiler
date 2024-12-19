use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct Input {
    term_ranges: Vec<(usize, usize)>,
    op_offsets: Vec<usize>,
}

pub fn demarcate(input: &str) -> Input {
    let mut char_stream = input.chars().enumerate().peekable();

    let mut term_ranges = Vec::new();
    let mut op_offsets = Vec::new();

    let ops = vec!['+', '-', '/', '*'];

    while let Some((start_idx, char)) = char_stream.next() {
        if char.is_numeric() {
            let mut offset = 0usize;
            while char_stream.peek().is_some_and(|x| x.1.is_numeric()) {
                char_stream.next();
                offset += 1;
            }

            term_ranges.push((start_idx, start_idx + offset));
        } else if ops.contains(&char) {
            op_offsets.push(start_idx);
        }
    }

    Input {
        term_ranges,
        op_offsets,
    }
}

pub fn parse(input: &str) -> isize {
    let demarcated_input = demarcate(input);

    if demarcated_input.term_ranges.len() == 0 {
        println!("NO VALID INPUT PROVIDED");
        return isize::MIN;
    }

    let mut term_stream = demarcated_input.term_ranges.iter().peekable();
    let mut op_stream = demarcated_input.op_offsets.iter();

    let term = term_stream.next().unwrap();
    let result = &mut input[term.0..=term.1].parse::<isize>().unwrap();

    while let Some(term) = term_stream.next() {
        let term = &mut input[term.0..=term.1].parse::<isize>().unwrap();
        let op_offset = *op_stream.next().unwrap();

        match &input[op_offset..=op_offset] {
            "+" => *result += *term,
            "-" => *result -= *term,
            "*" => *result *= *term,
            "/" => *result /= *term,
            _ => unreachable!(),
        }
    }

    *result
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
