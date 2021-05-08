use std::str::Chars;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Kind {
    Lparen, Rparen, Plus, Minus, Multi, Divi, Equal, NotEq,
    Less, LessEq, Greater, GreaterEq, SngQ, DblQ, Assign, Semicolon,
    If, Else, Print, Ident, IntNum,
    String, Letter, Digit, Nulkind, EofTkn, Others, Endlist,
    Lbrace, Rbrace
}

#[derive(Debug)]
pub struct Token{
    text: String,
    kind: Kind,
    val: i32,
}
impl Token {
    pub fn new() -> Token {
        Token{text: "".to_string(), kind: Kind::Others, val: 0}
    }
}
pub struct KeyWd<'a> {
    val: &'a str,
    kind: Kind
}

const KEY_WD_TBL: [KeyWd; 20] = [
    KeyWd{val: "(", kind: Kind::Lparen},
    KeyWd{val: ")", kind: Kind::Rparen},
    KeyWd{val: "{", kind: Kind::Lbrace},
    KeyWd{val: "}", kind: Kind::Rbrace},
    KeyWd{val: "+", kind: Kind::Plus},
    KeyWd{val: "-", kind: Kind::Minus},
    KeyWd{val: "*", kind: Kind::Multi},
    KeyWd{val: "/", kind: Kind::Divi},
    KeyWd{val: "==", kind: Kind::Equal},
    KeyWd{val: "!=", kind: Kind::NotEq},
    KeyWd{val: "<", kind: Kind::Less},
    KeyWd{val: "<=", kind: Kind::LessEq},
    KeyWd{val: ">", kind: Kind::Greater},
    KeyWd{val: ">=", kind: Kind::GreaterEq},
    KeyWd{val: "=", kind: Kind::Assign},
    KeyWd{val: ";", kind: Kind::Semicolon},
    KeyWd{val: "", kind: Kind::Endlist},
    KeyWd{val: "if", kind:Kind::If},
    KeyWd{val: "else", kind: Kind::Else},
    KeyWd{val:  "print", kind: Kind::Print},
    ]; // todo


#[derive(PartialEq, Debug, Copy, Clone, )]
pub enum Ch {
    Others, Digit, Letter, Assign, Lparen, Rparen, Less, Great,
    Plus, Minus, Multi, Divi, SngQ, DblQ, Semicolon, Lbrace, Rbrace,
    EOF,
}

pub fn init_ch_type() -> [Ch;256]{
    let mut ch_list:[Ch;256] = [Ch::Others;256];
    for i in '0' as usize ..='9' as usize {
        ch_list[i] = Ch::Digit;
    }
    for i in 'A' as usize ..= 'Z' as usize {
        ch_list[i] = Ch::Letter;
    }
    for i in 'a' as usize ..= 'z' as usize {
        ch_list[i] = Ch::Letter;
    }
    ch_list['_' as usize] = Ch::Letter;
    ch_list['=' as usize] = Ch::Assign;
    ch_list['(' as usize] = Ch::Lparen;
    ch_list[')' as usize] = Ch::Rparen;
    ch_list['{' as usize] = Ch::Lbrace;
    ch_list['}' as usize] = Ch::Rbrace;
    ch_list['<' as usize] = Ch::Less;
    ch_list['>' as usize] = Ch::Great;
    ch_list['+' as usize] = Ch::Plus;
    ch_list['-' as usize] = Ch::Minus;
    ch_list['*' as usize] = Ch::Multi;
    ch_list['/' as usize] = Ch::Divi;
    ch_list['\'' as usize] = Ch::SngQ;
    ch_list['"' as usize] = Ch::DblQ;
    ch_list[';' as usize] = Ch::Semicolon;

    ch_list
}



pub fn tokenize(text: &mut Chars) -> Vec<Token> {
    let mut tkn_res = vec![];
    let ch_list:[Ch;256] = init_ch_type();
    while true {
        tkn_res.push(next_tkn(text, &ch_list));
        if tkn_res.last().unwrap().kind == Kind::Endlist {
            break;
        }
    }
    tkn_res
}

fn next_tkn(text: &mut Chars, ch_list:&[Ch;256]) -> Token {
    let mut ch = ' ';
    let mut token = Token::new();
    while ch == ' ' || ch == '\n' {
        ch = next_ch(text);
    }

    if ch == '\0' {
        return Token { text: "".to_string(), kind: Kind::Endlist, val: 0 }
    }

    match ch_list[ch as usize] {
        Ch::Letter => {
            let mut s = "".to_string();
            while ch_list[ch as usize] == Ch::Letter || ch_list[ch as usize] == Ch::Digit {
                s = s + &ch.to_string();
                ch = next_ch(text);
                // todo 文字数制限
            }
            for word in KEY_WD_TBL.iter() {
                if &*s == word.val {
                    token.kind = word.kind;
                    token.text = s.clone();
                    break;
                }
            }
            if &token.text == "" {
                token.kind = Kind::Ident;
                token.text = s.clone();
            }

        },

        Ch::Digit => {
            let mut s:String = "".to_string();
            while ch_list[ch as usize] == Ch::Digit {
                s = s + &ch.to_string();
                ch = next_ch(text);
                // todo 文字数制限
            }
            if ch_list[ch as usize] == Ch::Letter { parse_error(); }
            token.kind = Kind::Digit;
            token.val = s.parse().unwrap();
        },
        
        _ => {

        }
    }
    token
}



fn next_ch(text: &mut Chars) -> char{
    let mut ch = match text.next() {
        None => return '\0',
        Some(h) => h
    };

    if ch == '/'  {
        ch = match text.next() {
            None => return '\0',
            Some(h) => h,
        };
        if ch == '/' {
            while ch != '\n' {
                ch = match text.next() {
                    None => return '\0',
                    Some(h) => h,
                }
            }
        } else {
            parse_error();
        }
    }

    ch
}

fn parse_error() {
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

    let token_list = tokenize(&mut text.chars());

    println!("text\tkind\tval");
    for tkn in token_list {
        println!("{}\t{:?}\t{}", tkn.text, tkn.kind, tkn.val);
    }
}