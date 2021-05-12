use std::cell::Cell;
use std::collections::HashMap;
use super::*;

pub fn token_to_expr(token_list: &Vec<Token>) -> Vec<Expr> {
    let mut expr_list:Vec<Expr> = Vec::new();
    let mut stack: Vec<Expr> = Vec::new();
    let mut id_map: HashMap<String, Expr> = HashMap::new();
    let mut i = 0;
    while i < token_list.len() - 1 {
        expr_list.push(statement(&mut i, token_list, &mut stack, &mut id_map));
    }
    expr_list
}


fn statement(index: &mut usize, token_list: &Vec<Token>, stack: &mut Vec<Expr>, id_map: &mut HashMap<String, Expr>) -> Expr {
    let token:&Token = match token_list.get(*index) {
        None => return Expr::Nope,
        Some(tkn) => tkn,
    };
    let cell_token:Cell<&Token> = Cell::new(token);

    match token.kind {
        Kind::Ident => { // todo 変数宣言の宣言(letなど)を認識
            let variable_name: String = token.text.clone();
            // ここでexpr::Identをstackにpush 今回は変数宣言だけでstackは使わない？
            let ident = Expr::Ident(Box::new(Ident::new(variable_name.clone(), Kind::Int)));
            next_tkn(&cell_token, index, token_list);
            check_tkn(&cell_token, index, token_list, Kind::Assign, "= is missing".to_string(), true); // todo Addasgn などに対応させる
            // ここでExpr::Assignを宣言 left->先程のexpr::Ident, right->expressionの返り値？
            expression(&cell_token, index, token_list, stack, id_map);
            let right_expr = match stack.pop() {
                None => {parse_error("stack pop failed: stack is empty".to_string()); return Expr::Nope; },
                Some(expr) => expr,
            };
            let assign: Expr = Expr::Assign(Box::new(Assign::new(
                    ident, 
                    right_expr.clone(),
            )));
            check_tkn(&cell_token, index, token_list, Kind::Semicolon, "; is missing".to_string(), true);
            id_map.insert(variable_name, right_expr);
            return assign;
        },
        Kind::Print => {
            
            next_tkn(&cell_token, index, token_list);
            check_tkn(&cell_token, index, token_list, Kind::Lparen, "( is missing for print function".to_string(), true);
            expression(&cell_token, index, token_list, stack, id_map);
            let print = Expr::Print(Box::new(Print::new(match stack.pop() {
                None => {parse_error("print error; stack is empty".to_string()); return Expr::Nope; }, // todo error messageを変更
                Some(expr) => expr
            })));
            check_tkn(&cell_token, index, token_list, Kind::Rparen, ") is missing for print function".to_string(), true);
            check_tkn(&cell_token, index, token_list, Kind::Semicolon, "; is missing".to_string(),true);
            return print
        }
        _ => {

        }
    }
    Expr::Nope
}

#[allow(unused_assignments)]
fn expression<'a>(cell_token:&'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>, id_map: &mut HashMap<String, Expr>) {
    let mut op: Kind = Kind::Nulkind;
    term(cell_token, index, token_list, stack, id_map);
    let mut token: &Token = cell_token.get();
    while token.kind == Kind::Plus || token.kind == Kind::Minus {
        op = token.kind;
        next_tkn(cell_token, index, token_list);
        term(cell_token, index, token_list, stack, id_map);
        operate(op, stack);
        token = cell_token.get();
    }
}

#[allow(unused_assignments)]
fn term<'a>(cell_token: &'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>, id_map: &mut HashMap<String, Expr>) {
    let mut op: Kind = Kind::Nulkind;
    factor(cell_token, index, token_list, stack, id_map);
    let mut token: &Token = cell_token.get();
    while token.kind == Kind::Multi || token.kind == Kind::Divi {
        op = token.kind;
        next_tkn(cell_token, index, token_list);
        factor(cell_token, index, token_list, stack, id_map);
        operate(op, stack);
        token = cell_token.get();
    }
}

fn factor<'a>(cell_token: &'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, stack: &mut Vec<Expr>, id_map: &mut HashMap<String, Expr>) {
    let token: &Token = cell_token.get();
    match token.kind {
        Kind::Ident => {
            stack.push(id_map[&token.text].clone());
        },
        Kind::Int => {
            stack.push(Expr::Int(Int::new(token.val)));
        },
        Kind::Lparen => {
            next_tkn(cell_token, index, token_list);
            expression(cell_token, index, token_list, stack, id_map);
            check_tkn(cell_token, index, token_list, Kind::Rparen, ") is missing".to_string(), false);
            
        },
        _ => {
            
        }
    }
    next_tkn(cell_token, index, token_list);
}


fn check_tkn<'a>(cell_token:&'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>, tp: Kind, message:String, next: bool) {
    let token:&Token = cell_token.get();
    if token.kind != tp {
        println!("error: {}", message);
        std::process::exit(1);
    }
    if next {
        next_tkn(cell_token, index, token_list);
    }
}

#[allow(unused_assignments)]
fn next_tkn<'a>(cell_token: &'a Cell<&'a Token>, index: &mut usize, token_list: &'a Vec<Token>) {
    let mut token = cell_token.get();
    *index += 1;
    token = match token_list.get(*index) {
        None => return,
        Some(tkn) => tkn
    };
    cell_token.set(token);
}

fn operate(op: Kind, stack: &mut Vec<Expr>) {
    let d2: Expr = match stack.pop() {
        None => { parse_error("error: stack is empty".to_string()); return;},
        Some(expr) => expr
    };
    let d1 = match stack.pop() {
        None => { parse_error("error: stack is empty".to_string()); return;},
        Some(expr) => expr
    };
    stack.push(Expr::BinaryOp(Box::new(BinaryOp::new(op, d1, d2))));
}  


fn parse_error(message: String) {
    println!("{}", message);
    println!("NG");
    std::process::exit(1);
}

