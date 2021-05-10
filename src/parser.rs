mod tokenizer;
use tokenizer::{Kind, Token, /* KeyWd, KEY_WD_TBL */};

#[derive(Debug, Clone)]
pub enum Expr {
    Int(Int),
    BinaryOp(Box<BinaryOp>),
    Nope
}
impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Int(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval(),
            Expr::Nope => 0
        }
    }
}
#[derive(Debug, Copy, Clone)]

pub struct Int(i32);
impl Int {
    pub fn new(val: i32) -> Int {
        Int(val)
    }
    pub fn eval(&self) -> i32 {
        self.0
    }
}
#[derive(Debug, Clone)]
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


pub fn token_to_expr(token_list: &Vec<Token>) -> Expr{
    let mut stack:Vec<Expr> = Vec::new();
    let mut root:Expr = Expr::Nope;
    let mut index:usize = 0;
    /*
while index < token_list.len() {
    println!("idnex: {}",index);
    root = match next_tkn(&mut index, token_list, &mut stack) {
        None => {
            root
        },
        Some(expr) => expr
    };
    index += 1;
}
*/
    root = match next_tkn(&mut index, token_list, &mut stack) {
        None => {
            root
        },
        Some(expr) => expr
    };
    root
}

pub fn next_tkn(index: &mut usize, token_list: &Vec<Token>, stack: &mut Vec<Expr>) -> Option<Expr> {
    let token:&Token = match token_list.get(*index) {
        None => return None,
        Some(h) => h,
    };
    println!("{:?} {}", token.kind, token.val);
    match token.kind {
        Kind::Int => {
{ 
                stack.push(Expr::Int(Int::new(token.val)));
                *index += 1;
                next_tkn(index, &token_list, stack)
            }
        },

        Kind::Plus | Kind::Minus | Kind::Multi | Kind::Divi => {
            let left:Expr = match stack.pop() {
                None => {
                    parse_error("stack pop failed: stack is empty".to_string());
                    return None;
                },
                Some(h) => h,
            };
            println!("left {:?}", left);
            let right:Expr = match next_tkn(&mut (*index + 1), &token_list, stack) {
                None => {
                    //parse_error("operand not found".to_string());
                    return None;
                },
                Some(h) => h,
            };
            *index += 1;
            let binary_op: Box<BinaryOp> = Box::new(BinaryOp::new(
                        token.kind,
                        left,
                        right
            ));
            let binary_op2:Box<BinaryOp> = binary_op.clone();
            stack.push(Expr::BinaryOp(binary_op));
            return Some(Expr::BinaryOp(binary_op2));

        }
        _ => {
            *index += 1;
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
    let res = token_to_expr(&token_list);
    println!("{:?}", res);
    println!("res = {}", res.eval());

}


#[allow(unused_macros)]
macro_rules! tst {
    ($x:expr) => {token_to_expr( &tokenizer::tokenize(&mut $x.to_string().chars())).eval()}
}

#[allow(unused_imports)]
use rand::Rng;

#[test]
fn parser_test() {
    assert!(tst!("1 + 1") == 2);
    assert!(tst!("1 + 2 - 3") == 0);
    assert!(tst!("2*3+4" ) == 10);
    assert!(tst!("2+3*4") == 14);
    assert!(tst!("1*2+3*4") == 14);
    //assert!(tst!("100 + 99*31-20+ 19 / 19") == 3150);
    /*
    for i in 0..100 {
        let mut s: String = String::new();
        let mut rng = rand::thread_rng();
        let limit: i32 = rng.gen() % 1000;
        let mut ans: i32;
        for i in 0..limit {
            let num: i32 = rng.gen() % 1000;
            let expr: i32 = rng.gen() % 4;
            if i == 0 { ans = num; s = num.to_string(); }
            if i != limit - 1{
                match expr {
                    0 => {
                        s = s + &'+'.to_string();
                        num += 
                    }
                }
            }
        }
    }
    */
}