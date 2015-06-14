use std::io;
use std::io::prelude::*;
use std::fmt::{Debug,Formatter,Error};
use std::str::FromStr;

fn main() {
    for n in 1.. {
        print!("[{}]> ", n);
        io::stdout().flush().unwrap();
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).unwrap();
        if cmd == "" { break; }  // ^D pressed

        println!("{:?}", tokenize(cmd.trim().to_string()));
    }
}

trait FromString<T> { fn from_string(String) -> Option<T>; }
enum Operator { Plus, Minus, Times, DividedBy, OpenParen, CloseParen,
    OpenBracket, CloseBracket, Comma }
// apparently there's no cleaner way to do this
impl ToString for Operator {
    fn to_string(&self) -> String {
        match *self {
            Operator::Plus => "+", Operator::Minus => "-",
            Operator::Times => "*", Operator::DividedBy => "/",
            Operator::OpenParen => "(", Operator::CloseParen => ")",
            Operator::OpenBracket => "[", Operator::CloseBracket => "]",
            Operator::Comma => ","
        }.to_string()
    }
}
impl FromString<Operator> for Operator {
    fn from_string(s: String) -> Option<Operator> {
        match &s[0..1] {
            "+" => Some(Operator::Plus),
            "-" => Some(Operator::Minus),
            "*" => Some(Operator::Times),
            "/" => Some(Operator::DividedBy),
            "(" => Some(Operator::OpenParen),
            ")" => Some(Operator::CloseParen),
            "[" => Some(Operator::OpenBracket),
            "]" => Some(Operator::CloseBracket),
            "," => Some(Operator::Comma),
            _ => None
        }
    }
}
enum Function { Range, Floor, Ceil }
enum Token {
    XNumber(f32),
    XString(String),
    XArray(Vec<Token>),
    XOperator(Operator),
    XFunction(Function)
}
impl Debug for Token {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Token::XNumber(ref n) => {
                write!(fmt, "{}", n);
            },
            Token::XString(ref s) => {
                write!(fmt, "{}", s);
            },
            Token::XArray(ref a) => {
                write!(fmt, "{:?}", a);
            },
            Token::XOperator(ref o) => {
                write!(fmt, "{}", (*o).to_string());
            },
            Token::XFunction(ref f) => {
                write!(fmt, "{}", match *f {
                    Function::Range => "range", Function::Floor => "floor",
                    Function::Ceil => "ceil"
                });
            }
        }
        Ok(())
    }
}

fn tokenize(cmd: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut pos: usize = 0;
    while pos < cmd.len() {
        let op_str = Operator::from_string(cmd.chars().skip(pos)
            .collect::<String>());
        if cmd.chars().nth(pos).unwrap().is_digit(10) {
            let num = cmd.chars().skip(pos).take_while(|c| c.is_digit(10))
                .collect::<String>();
            tokens.push(Token::XNumber(f32::from_str(&num).unwrap()));
            pos += num.len();
        } else if &cmd[pos..pos+1] == "\"" {
            let endquote = cmd.rfind('"').unwrap() + 1;
            tokens.push(Token::XString(cmd[pos..endquote].to_string()));
            pos = endquote;
        } else if op_str.is_some() {
            tokens.push(Token::XOperator(op_str.unwrap()));
            pos += 1;
        } else if pos+5 <= cmd.len() && &cmd[pos..pos+5] == "range" {
            tokens.push(Token::XFunction(Function::Range));
            pos += 5;
        } else if pos+5 <= cmd.len() && &cmd[pos..pos+5] == "floor" {
            tokens.push(Token::XFunction(Function::Floor));
            pos += 5;
        } else if pos+4 <= cmd.len() && &cmd[pos..pos+4] == "ceil" {
            tokens.push(Token::XFunction(Function::Ceil));
            pos += 4;
        } else if &cmd[pos..pos+1] == " " || &cmd[pos..pos+1] == "\t" {
            pos += 1;
        } else {
            panic!("Syntax error");
        }
    }
    tokens
}
