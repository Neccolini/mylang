
mod compiler;
use compiler::*;
use std::env;
use std::fs;

#[allow(dead_code)]
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
    for token in token_list.clone() {
        println!("{:?}", token);
    }
    let ast = parser::token_to_expr(&token_list);
    
    
    for expr in ast.clone() {
        println!("{:?}", expr);
    }
    
    compiler::generate(&ast);
    
}
