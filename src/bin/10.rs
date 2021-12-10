fn main() {
    let input = advent_of_code_2021::input("10");
    let sum = input
        .lines()
        .filter_map(|l| parse_chunk(l).err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum::<i32>();
    println!("{sum}");
}

#[derive(PartialEq, Eq)]
enum Brace {
    Open(BraceKind),
    Close(BraceKind),
}

impl Brace {
    fn from_char(c: char) -> Option<Self> {
        let kind = match c {
            '<' | '>' => BraceKind::Pointy,
            '{' | '}' => BraceKind::Curly,
            '[' | ']' => BraceKind::Square,
            '(' | ')' => BraceKind::Round,
            _ => return None,
        };
        Some(match c {
            '<' | '{' | '[' | '(' => Brace::Open(kind),
            '>' | '}' | ']' | ')' => Brace::Close(kind),
            _ => return None,
        })
    }

    fn to_char(&self) -> char {
        use Brace::*;
        use BraceKind::*;
        match self {
            Open(Pointy) => '<',
            Open(Curly) => '{',
            Open(Square) => '[',
            Open(Round) => '(',
            Close(Pointy) => '>',
            Close(Curly) => '}',
            Close(Square) => ']',
            Close(Round) => ')',
        }
    }
}

#[derive(PartialEq, Eq)]
enum BraceKind {
    Pointy,
    Curly,
    Square,
    Round,
}

fn parse_chunk(line: &str) -> Result<(), char> {
    let tokens = line.chars().filter_map(Brace::from_char);
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Brace::Open(kind) => stack.push(kind),
            Brace::Close(_) => expect(&mut stack, token)?,
        }
    }
    Ok(())
}

fn expect(stack: &mut Vec<BraceKind>, brace: Brace) -> Result<(), char> {
    let found = brace.to_char();
    let expected_kind = stack.pop().ok_or(found)?;
    let (Brace::Open(kind) | Brace::Close(kind)) = brace;
    if kind == expected_kind {
        Ok(())
    } else {
        Err(found)
    }
}
