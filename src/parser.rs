mod tokenizer;
use tokenizer::{Kind, Token, /* KeyWd, KEY_WD_TBL */};

#[derive(Debug)]
pub enum Expr {
    Int(Int),
    BinaryOp(Box<BinaryOp>)
}
impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Int(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval()
        }
    }
}
#[derive(Debug)]

pub struct Int(i32);
impl Int {
    pub fn new(val: i32) -> Int {
        Int(val)
    }
    pub fn eval(&self) -> i32 {
        self.0
    }
}
#[derive(Debug)]
pub struct BinaryOp {
    kind: Kind,
    left_expr: Expr,
    right_expr: Expr
}
impl BinaryOp {
    pub fn new(kind: Kind, left_expr:Expr, right_expr: Expr)-> BinaryOp {
        BinaryOp {kind, left_expr, right_expr}
    }
    pub fn eval(&self) -> i32 {
        let right = self.right_expr.eval();
        let left = self.left_expr.eval();
        match self.kind {
            Kind::Plus => left + right,
            Kind::Minus => left - right,
            Kind::Multi => left * right,
            Kind::Divi => left / right,
            _ => 0,
        }
    }
}


pub fn token_to_expr(token_list: &Vec<Token>) {
    let mut stack:Vec<Expr> = Vec::new();
    println!("{:?}",next_tkn(0, token_list, &mut stack));
    println!("OK?");
}

pub fn next_tkn(index: usize, token_list: &Vec<Token>, stack: &mut Vec<Expr>) -> Option<Expr> {
    let token:&Token = match token_list.get(index) {
        None => return None,
        Some(h) => h,
    };
    println!("{:?} {}", token.kind, token.val);
    match token.kind {
        Kind::Int => {
            stack.push(Expr::Int(Int::new(token.val)));
            next_tkn(index + 1, &token_list, stack)
        },

        Kind::Plus | Kind::Minus | Kind::Multi | Kind::Divi => {
            let left:Expr = match stack.pop() {
                None => {
                    parse_error("stack pop failed: stack is empty".to_string());
                    return None;
                },
                Some(h) => h,
            };
            let right:Expr = match next_tkn(index + 1, &token_list, stack) {
                None => {
                    parse_error("operand not found".to_string());
                    return None;
                },
                Some(h) => h,
            };
            let binary_op: Box<BinaryOp> = Box::new(BinaryOp::new(
                        token.kind, 
                        left,
                        right
            ));
            return Some(Expr::BinaryOp(binary_op));

        }
        _ => {
            return stack.pop()
        }
    }
}


fn parse_error(message: String) {
    println!("{}", message);
    println!("NG");
    std::process::exit(1);
}



use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("identify the filename to parse");
        std::process::exit(1);
    }

    // todo(入力ファイルが大きいと失敗する可能性がある)
    let text = match fs::read_to_string(&args[1]) {
        Ok(n) => n,
        Err(err) => {
            println!("error : {}", err);
            std::process::exit(1);
        }
    };

    let token_list = tokenizer::tokenize(&mut text.chars());
/*
    println!("text\tkind\tval");
    for tkn in token_list {
        println!("{}\t{:?}\t{}", tkn.text, tkn.kind, tkn.val);
    }
*/
    token_to_expr(&token_list);

}