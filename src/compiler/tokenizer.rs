use std::str::Chars;
use super::*;
pub const KEY_WD_TBL: [KeyWd; 25] = [
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
    KeyWd{val: "+=", kind: Kind::Addasgn},
    KeyWd{val: "-=", kind: Kind::Mnuasgn},
    KeyWd{val: "*=", kind: Kind::Multiasgn},
    KeyWd{val: "/=", kind: Kind::Divasgn},
    KeyWd{val: ";", kind: Kind::Semicolon},
    KeyWd{val: "", kind: Kind::Endlist},
    KeyWd{val: "if", kind:Kind::If},
    KeyWd{val: "else", kind: Kind::Else},
    KeyWd{val:  "print", kind: Kind::Print},
    KeyWd{val: "nyaan", kind: Kind::Nyaan},
    ]; // todo

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
    ch_list['>' as usize] = Ch::Greater;
    ch_list['+' as usize] = Ch::Plus;
    ch_list['-' as usize] = Ch::Minus;
    ch_list['*' as usize] = Ch::Multi;
    ch_list['/' as usize] = Ch::Divi;
    ch_list['\'' as usize] = Ch::SngQ;
    ch_list['"' as usize] = Ch::DblQ;
    ch_list[';' as usize] = Ch::Semicolon;
    ch_list['!' as usize] = Ch::Exclam;

    ch_list
}


#[allow(while_true)]
pub fn tokenize(text: &mut Chars) -> Vec<Token> {
    let mut tkn_res = vec![];
    let ch_list:[Ch;256] = init_ch_type();
    let mut prev_ch = ' ';
    while true {
        tkn_res.push(next_tkn(text, &ch_list, &mut prev_ch));
        if tkn_res.last().unwrap().kind == Kind::Endlist {
            break;
        }
    }
    tkn_res
}

fn next_tkn(text: &mut Chars, ch_list:&[Ch;256], prev_ch: &mut char) -> Token {
    let mut ch:char = *prev_ch;
    let mut token:Token = Token::new();
    while ch == ' ' || ch == '\n' {
        ch = next_ch(text);
    }

    if ch == '\0' {
        return Token { text: "".to_string(), chr:' ', kind: Kind::Endlist, val: 0 }
    }
    match ch_list[ch as usize] {
        Ch::Letter => {
            let mut s: String = "".to_string();
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
                token.text = s;
            }
            *prev_ch = ch;
        },

        Ch::Digit => {
            let mut s:String = "".to_string();
            while ch_list[ch as usize] == Ch::Digit {
                s = s + &ch.to_string();
                ch = next_ch(text);
                // todo 文字数制限
            }
            if ch_list[ch as usize] == Ch::Letter { parse_error("error at: Digit ".to_string() + &ch.to_string()); }
            token.kind = Kind::Int;
            token.val = s.parse().unwrap();
            *prev_ch = ch;
        },
        Ch::SngQ => {
            let c: char = next_ch(text);
            if next_ch(text) != '\'' { parse_error("error at: SngQ ".to_string() + &ch.to_string()); }
            token.kind = Kind::Char;
            token.chr = c;
        },

        Ch::DblQ => {
            let mut s: String = "".to_string();
            ch = next_ch(text);
            while ch != '"' {
                s = s + &ch.to_string();
                ch = next_ch(text);
                if ch == '\0' { parse_error("error at: DblQ ".to_string() + &ch.to_string());  }
            }
            token.kind = Kind::String;
            token.text = s;
            *prev_ch = next_ch(text);
        },

        // = + ! < > 
        Ch::Assign | Ch::Plus | Ch::Exclam | Ch::Less | Ch::Greater =>  {
            let nch = next_ch(text);
            if ch == '=' && nch == '=' {
                token.kind = Kind::Equal;
            }
            else if ch == '!' && nch == '=' {
                token.kind = Kind::NotEq;
            }
            else if ch == '+' && nch == '=' {
                token.kind = Kind::Addasgn;
            }
            else if ch == '-' && nch == '=' {
                token.kind = Kind::Mnuasgn;
            }
            else if ch == '*' && nch == '=' {
                token.kind = Kind::Multiasgn;
            }
            else if ch == '/' && nch == '=' {
                token.kind = Kind::Divasgn;
            }
            else if ch == '<' && nch == '=' {
                token.kind = Kind::LessEq;
            }
            else if ch == '>' && nch == '=' {
                token.kind = Kind::GreaterEq;
            }
            else {
                // todo できればfor文じゃなく書けるようにしたい
                for word in KEY_WD_TBL.iter() {
                    if ch.to_string() == word.val {
                        token.kind = word.kind;
                        break;
                    }
                }
                *prev_ch = nch;
                return token;
            }
            *prev_ch = next_ch(text);
        }
        _ => {
            for word in KEY_WD_TBL.iter() {
                if ch.to_string() == word.val {
                    token.kind = word.kind;
                    break;
                }
            }
            *prev_ch = next_ch(text);
        }
    }
    token
}



fn next_ch(text: &mut Chars) -> char{
    let ch: char = match text.next() {
        None => return '\0',
        Some(h) => h
    };
    // todo commentを処理したい.....
    /*
    if ch == '/'  {

        println!("{}", nch);
        if nch == '/' {
            let mut nnch = match text.next() {
                None => return '\0',
                Some(h) => h,
            };
            while nnch != '\n' {
                nnch = match text.next() {
                    None => return '\0',
                    Some(h) => h,
                }
            }
            return nnch;
        } else {
            //parse_error("error at: comment ".to_string() + &ch.to_string());
        }
    }
    */
    ch
}

fn parse_error(message: String) {
    println!("{}", message);
    println!("NG");
    std::process::exit(1);
}
