use std::io;
use std::io::prelude::*;
use std::fmt::{Debug,Formatter,Error};

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

enum Operator { Plus, Minus, Times, DividedBy, OpenParen, CloseParen }
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
                write!(fmt, "{}", match *o {
                    Operator::Plus => "+", Operator::Minus => "-",
                    Operator::Times => "*", Operator::DividedBy => "/",
                    Operator::OpenParen => "(", Operator::CloseParen => ")"
                });
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
    vec![Token::XNumber(10f32), Token::XString("test".to_string()),
        Token::XArray(vec![Token::XNumber(42f32), Token::XNumber(1337f32)]),
        Token::XOperator(Operator::Minus), Token::XFunction(Function::Floor)]
}
