
#[derive(PartialEq, Debug)]
pub enum Kind {
    Lparen, Rparen, Plus, Minus, Multi, Divi, Equal, NotEq,
    Less, LessEq, Great, GreatEq, SngQ, DblQ, Assign, Semicolon,
    If, Else, Puts, Ident, IntNum,
    String, Letter, Digit, Nulkind, EofTkn, Others, ENDlist
}

#[derive(Debug)]
pub struct Token<'a, T> {
    text: &'a str,
    kind: Kind,
    val: T,
}
pub struct KeyWd<'a> {
    val: &'a str,
    kind: Kind
}

const KeyWdTbl: [KeyWd; 1] = [KeyWd{val: "if", kind:Kind::If}];

// 0: ’¼‘O‚É / ‚ð“Ç‚ñ‚¾ó‘Ô
// 1: /* ‚ð“Ç‚ñ‚Å‚¢‚éó‘Ô
// 2: 
fn nextCh(ch: char, state: i8) -> char{
    
}


fn nextTkn() -> Token<'static, i32> {


    let test = Token::<i32> {
        text: "if",
        kind: Kind::If,
        val: 0
    };
    test
}

fn tokenize() -> Vec<Token<'static, i32>>{




    let token_list = vec![];
    token_list
}
use std::env;

fn main() {
    if env::args().len() == 1 {
        std::process::exit(1);
    }
    println!("text\tkind\tval");
    let token_list = tokenize();
    for tkn in token_list {
        println!("{}\t{:?}\t{}", tkn.text, tkn.kind, tkn.val);
    }
}