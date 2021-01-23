use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Number(u64),
    ParenOpen,
    ParenClose,
    OpAdd,
    OpMul,
    WhiteSpace,
}

impl Token {
    fn value(self: &Self) -> u64 {
        match self {
            Token::Number(v) => *v,
            _ => panic!("Can't call value on token type {:?}", self),
        }
    }
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = vec![];
    for c in expr.chars() {
        let token = if c.is_digit(10) {
            Token::Number(c.to_digit(10).unwrap() as u64)
        } else {
            match c {
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                '*' => Token::OpMul,
                '+' => Token::OpAdd,
                ' ' => Token::WhiteSpace,
                _ => panic!("Unexpected character: {}", c),
            }
        };
        if token != Token::WhiteSpace {
            tokens.push(token);
        }
    }
    tokens
}

fn get_paren_tokens(tokens: &[Token], start: usize) -> &[Token] {
    let mut end = 0;
    let mut paren_ptr = start;
    let mut level = 1;
    while paren_ptr < tokens.len() {
        if tokens[paren_ptr] == Token::ParenOpen {
            level += 1;
        } else if tokens[paren_ptr] == Token::ParenClose {
            level -= 1;
            if level == 0 {
                end = paren_ptr - 1;
                break;
            }
        }
        paren_ptr += 1;
    }
    &tokens[start..=end]
}

fn execute(tokens: &[Token]) -> u64 {
    let mut right;
    let mut op;
    let mut ptr = 0;

    let mut left = tokens[0];
    while ptr < tokens.len() - 1 {
        if left == Token::ParenOpen {
            let paren_tokens = get_paren_tokens(tokens, ptr + 1);
            left = Token::Number(execute(paren_tokens));
            ptr += paren_tokens.len() + 1;
            if ptr >= tokens.len() - 1 {
                break;
            }
        }
        op = tokens[ptr + 1];
        right = tokens[ptr + 2];
        if right == Token::ParenOpen {
            let paren_tokens = get_paren_tokens(tokens, ptr + 3);
            right = Token::Number(execute(paren_tokens));
            ptr += paren_tokens.len() + 1;
        }
        // dbg!(left, op, right);
        left = match op {
            Token::OpAdd => Token::Number(left.value() + right.value()),
            Token::OpMul => Token::Number(left.value() * right.value()),
            _ => panic!("Unexpected operator: {:?} at {}", op, ptr + 1),
        };
        ptr += 2;
    }
    left.value()
}

fn eval(expr: &str) -> u64 {
    let tokens = tokenize(expr);
    // parse(&tokens);
    execute(&tokens)
}
#[aoc(day18, part1)]
fn solve_part1(inp: &str) -> u64 {
    inp.lines().map(|line| eval(line)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(eval("(1 + 2)"), 3);
        assert_eq!(eval("1 + (2 + 3)"), 6);
        assert_eq!(eval("((1 + 2) + (3 + 4))"), 10);
    }

    #[test]
    fn test_examples() {
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }
}
