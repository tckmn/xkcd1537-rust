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
        evaluate(tokenize(cmd.trim().to_string()));
    }
}

trait FromString<T> { fn from_string(String) -> Option<(T, usize)>; }
#[derive(PartialEq, Clone)]
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
    fn from_string(s: String) -> Option<(Operator, usize)> {
        match &s[0..1] {
            "+" => Some((Operator::Plus, 1)),
            "-" => Some((Operator::Minus, 1)),
            "*" => Some((Operator::Times, 1)),
            "/" => Some((Operator::DividedBy, 1)),
            "(" => Some((Operator::OpenParen, 1)),
            ")" => Some((Operator::CloseParen, 1)),
            "[" => Some((Operator::OpenBracket, 1)),
            "]" => Some((Operator::CloseBracket, 1)),
            "," => Some((Operator::Comma, 1)),
            _ => None
        }
    }
}
impl Operator {
    fn prec(&self) -> u8 {
        match *self {
            Operator::Plus => 1, Operator::Minus => 1,
            Operator::Times => 2, Operator::DividedBy => 2,
            _ => panic!("no precedence")
        }
    }
}
#[derive(PartialEq, Clone)]
enum Function { Range, Floor, Ceil }
impl ToString for Function {
    fn to_string(&self) -> String {
        match *self {
            Function::Range => "range", Function::Floor => "floor",
            Function::Ceil => "ceil"
        }.to_string()
    }
}
impl FromString<Function> for Function {
    fn from_string(s: String) -> Option<(Function, usize)> {
        if 5 <= s.len() && &s[0..5] == "range" {
            Some((Function::Range, 5))
        } else if 5 <= s.len() && &s[0..5] == "floor" {
            Some((Function::Floor, 5))
        } else if 4 <= s.len() && &s[0..4] == "ceil" {
            Some((Function::Ceil, 4))
        } else { None }
    }
}
#[derive(PartialEq, Clone)]
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
                try!(write!(fmt, "{}", n));
            },
            Token::XString(ref s) => {
                try!(write!(fmt, "{}", s));
            },
            Token::XArray(ref a) => {
                try!(write!(fmt, "{:?}", a));
            },
            Token::XOperator(ref o) => {
                try!(write!(fmt, "{}", (*o).to_string()));
            },
            Token::XFunction(ref f) => {
                try!(write!(fmt, "{}", (*f).to_string()));
            }
        }
        Ok(())
    }
}

fn tokenize(cmd: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut pos: usize = 0;
    while pos < cmd.len() {
        let c = cmd.chars().nth(pos).unwrap();
        let op_str = Operator::from_string(cmd.chars().skip(pos)
            .collect::<String>());
        let func_str = Function::from_string(cmd.chars().skip(pos)
            .collect::<String>());

        if c.is_digit(10) || c == '.' {
            let num = cmd.chars().skip(pos).take_while(|c| c.is_digit(10) ||
                *c == '.').collect::<String>();
            tokens.push(Token::XNumber(f32::from_str(&num).unwrap()));
            pos += num.len();
        } else if c == '"' {
            let endquote = cmd.rfind('"').unwrap() + 1;
            tokens.push(Token::XString(cmd[pos..endquote].to_string()));
            pos = endquote;
        } else if op_str.is_some() {
            let (s, len) = op_str.unwrap();
            tokens.push(Token::XOperator(s));
            pos += len;
        } else if func_str.is_some() {
            let (s, len) = func_str.unwrap();
            tokens.push(Token::XFunction(s));
            pos += len;
        } else if c == ' ' || c == '\t' {
            pos += 1;
        } else {
            panic!("Syntax error");
        }
    }
    tokens
}

fn evaluate(tokens: Vec<Token>) {
    let mut rpn: Vec<Token> = vec![];
    let mut opstack: Vec<Token> = vec![]; // must be Vec<Token> because can
                                          // contain Functions
    for token in tokens {
        match token {
            Token::XNumber(n) => { rpn.push(Token::XNumber(n)); },
            Token::XString(s) => { rpn.push(Token::XString(s)); },
            Token::XArray(_) => { panic!("array in tokens in evaluate()"); },
            Token::XOperator(o) => {
                match o {
                    Operator::Comma => {
                        while opstack[opstack.len()-1] !=
                                Token::XOperator(Operator::OpenParen) &&
                            opstack[opstack.len()-1] !=
                                Token::XOperator(Operator::OpenBracket) {
                            rpn.push(opstack.pop().unwrap());
                        }
                    },
                    Operator::OpenParen => { opstack.push(Token::XOperator(o)); },
                    Operator::CloseParen => {
                        // TODO
                    },
                    Operator::OpenBracket => { opstack.push(Token::XOperator(o)); },
                    Operator::CloseBracket => {
                        // TODO
                    },
                    _ => {
                        while let Some(_) = match opstack.last() {
                            Some(&Token::XOperator(ref o2)) =>
                                if o.prec() <= o2.prec() { Some(o2) }
                                else { None },
                            _ => None} {
                            rpn.push(opstack.pop().unwrap());
                        }
                        opstack.push(Token::XOperator(o));
                    }
                }
            },
            Token::XFunction(f) => { opstack.push(Token::XFunction(f)); }
        }
    }
    while let Some(op) = opstack.pop() {
        rpn.push(op);
    }

    println!("{:?}", rpn);
}
