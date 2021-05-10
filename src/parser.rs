mod tokenizer;
use tokenizer::{Kind, Token, /* KeyWd, KEY_WD_TBL */};
use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]

pub struct Int(i32);
impl Int {
    pub fn new(val: i32) -> Int {
        Int(val)
    }
    pub fn eval(&self) -> i32 {
        self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub fn token_to_expr(token_list: &Vec<Token>) -> Expr {
    let mut stack: Vec<Expr> = Vec::new();
    let mut id_map: HashMap<String, Expr> = HashMap::new();
    for mut i in 0..token_list.len() {
        statement(&mut i, token_list, &mut stack, &mut id_map);
    }
    Expr::Nope
}

fn statement(index: &mut usize, token_list: &Vec<Token>, stack: &mut Vec<Expr>, id_map: &mut HashMap<String, Expr>) {
    let mut token:&Token = match token_list.get(*index) {
        None => return,
        Some(tkn) => tkn,
    };
    let cell_token:Cell<&Token> = Cell::new(token);

    
    match token.kind {
        Kind::Ident => { // todo 変数宣言の宣言(letなど)を認識
            *index += 1;
            token = match token_list.get(*index) {
                None => return,
                Some(tkn) => tkn,
            };
            check_tkn(token,Kind::Assign, "= is missing".to_string());// todo Addasgn などに対応させる
            expression(&cell_token, index, token_list, stack);
            check_tkn(token, Kind::Semicolon, "; is missing".to_string());
            id_map.insert(token.text.clone(), match stack.pop() {
                None => {parse_error("stack pop failed: stack is empty".to_string()); return; },
                Some(expr) => expr,
            });
        },
        Kind::Print => {

        }
        _ => {
            
        }
    }
}

#[allow(unused_assignments)]
fn expression<'a>(cell_token:&'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>) {
    let mut op: Kind = Kind::Nulkind;
    let mut token:&Token = cell_token.get();
    term(cell_token, index, token_list, stack);
    while token.kind == Kind::Plus || token.kind == Kind::Minus {
        op = token.kind;
        *index += 1;
        token = match token_list.get(*index) {
            None => return,
            Some(tkn) => tkn,
        };
        cell_token.set(token);
        term(cell_token, index, token_list, stack);
        operate(op, stack);
    }
}

#[allow(unused_assignments)]
fn term<'a>(cell_token: &'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>) {
    let mut op: Kind = Kind::Nulkind;
    let mut token: &Token = cell_token.get();
    factor(cell_token, index, token_list, stack);
    while token.kind == Kind::Multi || token.kind == Kind::Divi {
        op = token.kind;
        *index += 1;
        token = match token_list.get(*index) {
            None => return,
            Some(tkn) => tkn,
        };
        cell_token.set(token);
        factor(cell_token, index, token_list, stack);
        operate(op, stack);
    }
}

fn factor<'a>(cell_token: &'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>) {
    let mut token = cell_token.get();
    match token.kind {
        Kind::Ident => {

        },
        Kind::Int => {
            stack.push(Expr::Int(Int::new(token.val)));
        },
        Kind::Lparen => {
            *index += 1;
            token = match token_list.get(*index) {
                None => return,
                Some(tkn) => tkn,
            };
            cell_token.set(token);
            expression(cell_token, index, token_list, stack);
            check_tkn(token, Kind::Rparen, ") is missing".to_string());
            
        },
        _ => {
            
        }
    }
    *index += 1;
    token = match token_list.get(*index) {
        None => return,
        Some(tkn) => tkn,
    };
    cell_token.set(token);
}


fn check_tkn(token: &Token, tp: Kind, message:String) {
    if token.kind != tp {
        println!("error: {}", message);
        std::process::exit(1);
    }
}


fn operate(op: Kind, stack: &mut Vec<Expr>) {
    let d2:Expr = match stack.pop() {
        None => { parse_error("error: stack is empty".to_string()); return;},
        Some(expr) => expr
    };
    let d1 = match stack.pop() {
        None => { parse_error("error: stack is empty".to_string()); return;},
        Some(expr) => expr
    };
    stack.push(Expr::BinaryOp(Box::new(BinaryOp::new(op, d1, d2))));
}  


/*
pub fn next_tkn(index: &mut usize, token_list: &Vec<Token>, stack: &mut Vec<Expr>) -> Option<Expr> {
    let token:&Token = match token_list.get(*index) {
        None => return None,
        Some(h) => h,
    };
    println!("{:?} {}", token.kind, token.val);
    match token.kind {
        Kind::Int => {
            if stack.is_empty() {
                stack.push(Expr::Int(Int::new(token.val)));
            }
            *index += 1;
            next_tkn(index, &token_list, stack)
            
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
            //let binary_op2:Box<BinaryOp> = binary_op.clone();
            if token.kind == Kind::Multi || token.kind == Kind::Divi {
                stack.push(Expr::BinaryOp(binary_op));
            }
            return next_tkn(index, token_list, stack);

        }
        _ => {
            *index += 1;
            return stack.pop()
        }
    }
}
*/

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