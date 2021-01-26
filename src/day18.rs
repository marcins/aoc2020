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

fn execute(tokens: &[Token], add_precedence: bool) -> u64 {
    let mut expanded_tokens: Vec<Token> = vec![];
    let mut level = 0;
    let mut start_paren = 0;

    // eliminates parens by taking a paren'd expression and passing it back through
    // execute to "reduce" it.
    for (idx, token) in tokens.iter().enumerate() {
        match token {
            Token::ParenOpen => {
                if level == 0 {
                    start_paren = idx;
                }
                level += 1;
            }
            Token::ParenClose => {
                level -= 1;
                if level == 0 {
                    let paren_value =
                        Token::Number(execute(&tokens[start_paren + 1..idx], add_precedence));
                    expanded_tokens.push(paren_value);
                }
            }
            _ => {
                if level == 0 {
                    expanded_tokens.push(*token)
                }
            }
        }
    }

    // At this point `updated_tokens` should be a simplified with parens evaluated

    // if we're doing add precedence, run through the expression and replace all the a + b with a value
    // then run through the expression again to do the muls, else just run through once and do whatever.
    if expanded_tokens.len() == 1 {
        return expanded_tokens[0].value();
    }

    let passes = if add_precedence { 2 } else { 1 };
    let mut input = expanded_tokens;
    let mut output = vec![];
    let mut left = input[0];
    for pass in 0..passes {
        let mut ptr = 0;
        left = input[0];
        while ptr < input.len() - 1 {
            let op = input[ptr + 1];
            let right = input[ptr + 2];
            if add_precedence && pass == 0 {
                match op {
                    Token::OpAdd => {
                        left = Token::Number(left.value() + right.value());
                        ptr += 2;
                    }
                    Token::OpMul => {
                        output.push(left);
                        output.push(op);
                        left = right;
                        ptr += 2;
                    }
                    _ => panic!("Unexpected op {:?}", op),
                }
            } else {
                left = match op {
                    Token::OpAdd => Token::Number(left.value() + right.value()),
                    Token::OpMul => Token::Number(left.value() * right.value()),
                    _ => panic!("Unexpected operator: {:?} at {}", op, ptr + 1),
                };
                ptr += 2;
            }
        }
        if pass == 0 && add_precedence {
            output.push(left);
        }
        input = output.to_owned()
    }
    left.value()
}

fn eval(expr: &str) -> u64 {
    let tokens = tokenize(expr);
    execute(&tokens, false)
}

fn eval_with_precendece(expr: &str) -> u64 {
    let tokens = tokenize(expr);
    execute(&tokens, true)
}

#[aoc(day18, part1)]
fn solve_part1(inp: &str) -> u64 {
    inp.lines().map(|line| eval(line)).sum()
}

#[aoc(day18, part2)]
fn solve_part2(inp: &str) -> u64 {
    inp.lines().map(|line| eval_with_precendece(line)).sum()
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

    #[test]
    fn test_examples_part2() {
        assert_eq!(eval_with_precendece("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(eval_with_precendece("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval_with_precendece("2 * 3 + (4 * 5)"), 46);
        assert_eq!(eval_with_precendece("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            eval_with_precendece("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            eval_with_precendece("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
