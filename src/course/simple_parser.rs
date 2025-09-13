use std::{collections::VecDeque, iter::Peekable};

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
pub struct Lexer {
    tokens: VecDeque<Token>,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        Self {
            tokens: to_tokens(s),
        }
    }

    pub fn next(&mut self) -> std::option::Option<Token> {
        self.tokens.pop_front()
    }
    pub fn peek(&mut self) -> std::option::Option<&Token> {
        self.tokens.front()
    }
}

pub fn to_expr_re<I: Iterator<Item = Token>>(iter: &mut Peekable<I>, min_power: f32) -> Expr {
    let first_token = iter.next().expect("Invalid token,token can't be empty");
    let mut left = match first_token {
        Token::Num(v) => Expr::Num(v),
        _ => {
            panic!("Invalid token {:?}", first_token)
        }
    };
    loop {
        // 查看先一个运算符的优先级，如果优先级高于前一个字符，当前数字与之结合，否则与前面的运算符结合并组成一个树
        let op_char = match iter.peek() {
            Some(token) => {
                match token {
                    Token::Op(c) => {
                        // 先peek查看运算符，如果是运算符才继续处理
                        Some(*c)
                    }
                    // 如果碰到了eof或数字，就截止到此处
                    Token::Eof => {
                        break;
                    }
                    Token::Num(_) => {
                        // 遇到数字，说明表达式结束
                        break;
                    }
                }
            }
            None => unreachable!("Invalid expression,Please check your expression is right?"),
        };

        // 如果没有运算符，就退出循环
        let op = match op_char {
            Some(c) => c,
            None => break,
        };

        // 消耗运算符token
        iter.next();
        // 获取下一个字符左侧优先级
        let (l_bp, r_bp) = get_power(op);
        // 如果当前运算符的优先级小于等于最小优先级，就退出
        if l_bp <= min_power {
            break;
        }
        let right = to_expr_re(iter, r_bp);
        left = Expr::Op(op, vec![left, right]);
    }
    left
}
pub fn get_power(c: char) -> (f32, f32) {
    match c {
        '+' | '-' => (1., 1.1),
        '*' | '/' => (2., 2.1),
        _ => panic!("Unknown character"),
    }
}
pub fn to_tokens(s: String) -> VecDeque<Token> {
    let mut v = VecDeque::new();
    for c in s.chars() {
        match c {
            '0'..='9' => {
                v.push_back(Token::Num(c));
            }
            '+' | '-' | '*' | '/' => {
                v.push_back(Token::Op(c));
            }
            _ => {
                panic!("Unknown character {}", c)
            }
        }
    }
    v.push_back(Token::Eof);
    v
}

#[test]
fn test() {
    let v = to_tokens("1+2*3+9".to_string());
    println!("Tokens: {:?}", v);

    let e = to_expr_re(&mut v.into_iter().peekable(), 0.);
    println!("Expression: {:?}", e);

    // 测试乘法优先级高于加法
    let v2 = to_tokens("1+2*3".to_string());
    let e2 = to_expr_re(&mut v2.into_iter().peekable(), 0.);
    println!("1+2*3 = {:?}", e2);

    // 测试加法结合性
    let v3 = to_tokens("1+2+3".to_string());
    let e3 = to_expr_re(&mut v3.into_iter().peekable(), 0.);
    println!("1+2+3 = {:?}", e3);

    // 测试括号效果（通过优先级模拟）
    let v4 = to_tokens("2*3+4".to_string());
    let e4 = to_expr_re(&mut v4.into_iter().peekable(), 0.);
    println!("2*3+4 = {:?}", e4);
}
