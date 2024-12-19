use std::error::Error;

type ParseResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MathOperation {
    Add,
    Sub,
    Mul,
    Div,
}

impl TryFrom<char> for MathOperation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(MathOperation::Add),
            '-' => Ok(MathOperation::Sub),
            '*' => Ok(MathOperation::Mul),
            '/' => Ok(MathOperation::Div),
            _ => Err(format!("{} IS AN UNDEFINED OPERATION", value)),
        }
    }
}

impl From<MathOperation> for char {
    fn from(value: MathOperation) -> Self {
        match value {
            MathOperation::Add => '+',
            MathOperation::Sub => '-',
            MathOperation::Mul => '*',
            MathOperation::Div => '/',
        }
    }
}

#[derive(Debug)]
pub struct TermInfo<'a> {
    content: &'a str,
    _span: core::range::Range<usize>,
}

impl TermInfo<'_> {
    pub fn new(content: &str, _span: core::range::Range<usize>) -> TermInfo {
        TermInfo { content, _span }
    }
}

#[derive(Debug)]
pub struct InputInfo<'a> {
    terms: Vec<TermInfo<'a>>,
    operations: Vec<MathOperation>,
}

impl InputInfo<'_> {
    pub fn new(terms: Vec<TermInfo>, operations: Vec<MathOperation>) -> InputInfo {
        InputInfo { terms, operations }
    }
}

pub fn demarcate(input: &str) -> ParseResult<InputInfo> {
    let mut istream = input.chars().enumerate().peekable();

    let mut terms = Vec::new();
    let mut operations = Vec::new();

    while let Some(char) = istream.next() {
        match char {
            (start, '0'..='9') => {
                let mut curr = char;
                while istream.peek().is_some_and(|x| x.1.is_numeric()) {
                    curr = istream
                        .next()
                        .expect("INPUT STREAM RETURNED NONE WHEN SOME");
                }

                let span = core::range::Range::from(start..(curr.0 + 1));
                terms.push(TermInfo::new(&input[span], span))
            }
            (_, '+' | '-' | '*' | '/') => operations.push(MathOperation::try_from(char.1)?),
            _ => {}
        }
    }

    Ok(InputInfo::new(terms, operations))
}

pub fn parse(input: &str) -> ParseResult<isize> {
    let demarcated_input = demarcate(input)?;

    if demarcated_input.terms.len() == 0 {
        return Err("NO VALID INPUT PROVIDED".into());
    }

    let mut term_stream = demarcated_input.terms.iter().peekable();
    let mut op_stream = demarcated_input.operations.iter();

    let term = term_stream.next().unwrap().content;
    let mut result = term.parse::<isize>().unwrap();

    while let Some(&ref term) = term_stream.next() {
        let term = term.content.parse::<isize>().unwrap();
        let operation = op_stream.next().unwrap();

        match operation {
            MathOperation::Add => result += term,
            MathOperation::Sub => result -= term,
            MathOperation::Mul => result *= term,
            MathOperation::Div => result /= term,
        }
    }

    Ok(result)
}
