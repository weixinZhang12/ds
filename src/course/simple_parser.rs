// 负责将表达式解析为token
#[derive(Debug)]
pub enum Token {
    Num(char),
    Op(char),
    Eof,
}
// 方便转换为树结构
#[derive(Debug)]
pub enum Expr {
    Num(char),
    Op(char, Vec<Expr>),
}

pub fn to_expr(v: Vec<Token>) -> Expr {
    let mut iter = v.into_iter();

    let left = iter.next().expect("Invalid token,token can't be empty");
    let left = match left {
        Token::Num(v) => Expr::Num(v),
        _ => {
            panic!("Invalid token {:?}", left)
        }
    };
    let op = iter.next().expect("Expecting a operator, but no operator");
    let op = match op {
        Token::Op(op) => op,
        Token::Eof => {
            return left;
        }
        _ => {
            panic!("Expecting a operator,but get '{:?}'.Please check your expression is complete.", op)
        }
    };
    let right = iter.next().expect("Invalid token,token can't be empty");
    let right = match right {
        Token::Num(right) => Expr::Num(right),
        _ => {
            panic!("Expecting a operator,but get '{:?}'.Please check your expression is complete.", right)
        }
    };
    Expr::Op(op, vec![left, right])
}

pub fn to_tokens(s: String) -> std::vec::Vec<Token> {
    let mut v = Vec::new();
    for c in s.chars() {
        match c {
            '0'..='9' => {
                v.push(Token::Num(c));
            }
            '+' | '-' | '*' | '/' => {
                v.push(Token::Op(c));
            }
            _ => {
                panic!("Unknown character {}", c)
            }
        }
    }
    v.push(Token::Eof);
    v
}

#[test]

fn test() {
    let v = to_tokens("1+2".to_string());
    println!("{:?}", v);

    let e = to_expr(v);
    println!("{:?}", e);
}
