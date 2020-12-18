use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    LeftParen,
    RightParen,
    Add,
    Mul,
    Num(i64),
}

struct Tokenizer<'a> {
    characters: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            characters: s.chars().peekable(),
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        // Skip any whitespace
        while self.characters.peek() == Some(&' ') {
            self.characters.next();
        }

        // Pull next character, bailing out if end of string.
        let c = self.characters.next();
        if c.is_none() {
            return None;
        }
        let c = c.unwrap();

        match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '+' => Some(Token::Add),
            '*' => Some(Token::Mul),
            '0'..='9' => {
                let mut val = (c as i64) - ('0' as i64);
                loop {
                    let peek = self.characters.peek();
                    if peek.is_none() {
                        break;
                    }
                    let peek = peek.unwrap();
                    if (peek >= &'0') && (peek <= &'9') {
                        val = val * 10 + ((*peek as i64) - ('0' as i64));
                        self.characters.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Num(val))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum ParseError {
    Unexpected {
        token: Option<Token>,
        expected: Option<Token>,
    },
}

impl From<ParseError> for util::Error {
    fn from(_e: ParseError) -> Self {
        util::Error::ParseError
    }
}

#[derive(Debug)]
enum Expr {
    Num(i64),
    Add { left: Box<Expr>, right: Box<Expr> },
    Mul { left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Num(x) => *x,
            Expr::Add { left, right } => left.eval() + right.eval(),
            Expr::Mul { left, right } => left.eval() * right.eval(),
        }
    }

    fn new_p1(tokens: Tokenizer) -> Result<Self, ParseError> {
        let mut tokens = tokens.peekable();
        let output = Self::expr_p1(&mut tokens);

        let token = tokens.next();
        match token {
            None => output,
            _ => Err(ParseError::Unexpected {
                token,
                expected: None,
            }),
        }
    }

    fn new_p2(tokens: Tokenizer) -> Result<Self, ParseError> {
        let mut tokens = tokens.peekable();
        let output = Self::expr_p2(&mut tokens);

        let token = tokens.next();
        match token {
            None => output,
            _ => Err(ParseError::Unexpected {
                token,
                expected: None,
            }),
        }
    }

    fn expr_p1(tokens: &mut Peekable<Tokenizer>) -> Result<Self, ParseError> {
        let mut current_expr = Self::factor_p1(tokens)?;

        loop {
            let next_token = tokens.peek();
            match next_token {
                Some(Token::Add) => {
                    tokens.next();
                    current_expr = Expr::Add {
                        left: Box::new(current_expr),
                        right: Box::new(Self::factor_p1(tokens)?),
                    }
                }

                Some(Token::Mul) => {
                    tokens.next();
                    current_expr = Expr::Mul {
                        left: Box::new(current_expr),
                        right: Box::new(Self::factor_p1(tokens)?),
                    }
                }

                _ => {
                    break;
                }
            }
        }

        Ok(current_expr)
    }

    fn expr_p2(tokens: &mut Peekable<Tokenizer>) -> Result<Self, ParseError> {
        let mut current_expr = Self::term_p2(tokens)?;

        loop {
            let next_token = tokens.peek();
            match next_token {
                Some(Token::Mul) => {
                    tokens.next();
                    current_expr = Expr::Mul {
                        left: Box::new(current_expr),
                        right: Box::new(Self::term_p2(tokens)?),
                    }
                }
                _ => {
                    break;
                }
            }
        }

        Ok(current_expr)
    }

    fn term_p2(tokens: &mut Peekable<Tokenizer>) -> Result<Self, ParseError> {
        let mut current_term = Self::factor_p2(tokens)?;

        loop {
            let next_token = tokens.peek();
            match next_token {
                Some(Token::Add) => {
                    tokens.next();
                    current_term = Expr::Add {
                        left: Box::new(current_term),
                        right: Box::new(Self::factor_p2(tokens)?),
                    }
                }
                _ => {
                    break;
                }
            }
        }

        Ok(current_term)
    }

    fn factor_p1(tokens: &mut Peekable<Tokenizer>) -> Result<Self, ParseError> {
        let token = tokens.next();
        match token {
            // Any of these cannot be at the start of a new factor
            None
            | Some(Token::Add)
            | Some(Token::Mul)
            | Some(Token::RightParen) => Err(ParseError::Unexpected {
                token,
                expected: Some(Token::Num(123)),
            }),

            // Numeric token becomes numeric expression
            Some(Token::Num(x)) => Ok(Expr::Num(x)),

            // Parenthesized expression gets recursed into, expects
            // right parentheses after.
            Some(Token::LeftParen) => {
                let inner = Self::expr_p1(tokens);
                let closing = tokens.next();
                match closing {
                    Some(Token::RightParen) => inner,
                    _ => Err(ParseError::Unexpected {
                        token: closing,
                        expected: Some(Token::RightParen),
                    }),
                }
            }
        }
    }

    fn factor_p2(tokens: &mut Peekable<Tokenizer>) -> Result<Self, ParseError> {
        let token = tokens.next();
        match token {
            // Any of these cannot be at the start of a new factor
            None
            | Some(Token::Add)
            | Some(Token::Mul)
            | Some(Token::RightParen) => Err(ParseError::Unexpected {
                token,
                expected: Some(Token::Num(123)),
            }),

            // Numeric token becomes numeric expression
            Some(Token::Num(x)) => Ok(Expr::Num(x)),

            // Parenthesized expression gets recursed into, expects
            // right parentheses after.
            Some(Token::LeftParen) => {
                let inner = Self::expr_p2(tokens);
                let closing = tokens.next();
                match closing {
                    Some(Token::RightParen) => inner,
                    _ => Err(ParseError::Unexpected {
                        token: closing,
                        expected: Some(Token::RightParen),
                    }),
                }
            }
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;

    let sum_p1 = text
        .lines()
        .map(|line| Expr::new_p1(Tokenizer::new(line)))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|expr| expr.eval())
        .sum::<i64>();
    println!("Part 1, sum = {}", sum_p1);

    let sum_p2 = text
        .lines()
        .map(|line| Expr::new_p2(Tokenizer::new(line)))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|expr| expr.eval())
        .sum::<i64>();
    println!("Part 2, sum = {}", sum_p2);

    Ok(())
}
