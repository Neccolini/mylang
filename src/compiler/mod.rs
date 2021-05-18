pub mod parser;
pub mod tokenizer;

use inkwell::{context::Context};
use inkwell::{AddressSpace, IntPredicate};
use inkwell::module::{Linkage};
use inkwell::values::{IntValue, PointerValue, BasicValueEnum};
use std::{collections::HashMap};
use std::cell::{RefCell, Cell};

#[allow(unused_mut)]
#[allow(non_camel_case_types)]
#[allow(unused_assignments)]
pub fn generate(ast: &Vec<Expr>) {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();
    let mut var_table_cell: RefCell<HashMap<String, PointerValue>> = RefCell::new(HashMap::new());
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let printf_type = i32_type.fn_type(&[str_type.into()], true);
    module.add_function("puts", printf_type, Some(Linkage::External));
    module.add_function("putchar", putchar_type, None);
    module.add_function("printf", printf_type, Some(Linkage::External));
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    let get_int_from_int_val = |int_val: IntValue| -> i32{
        let s = int_val.print_to_string().to_string();
        let vec:Vec<&str> = s.split_whitespace().collect();
        let val: i32 = vec[1].parse().unwrap();
        val
    };
    let int_cell: Cell<BasicValueEnum> = Cell::new(BasicValueEnum::IntValue(context.i32_type().const_int(0, false)));
    struct Eval_Int_Formula<'s>{ f: &'s dyn Fn(&Eval_Int_Formula, Expr) }
    let eval_int_formula= Eval_Int_Formula {
        f: &|eval_int_formula, expr| {
            match expr {
                Expr::Int(e) => {
                    int_cell.set(BasicValueEnum::IntValue(context.i32_type().const_int(e.eval() as u64, false)));
                },
                Expr::Ident(e) => {
                    let mut var_table = var_table_cell.borrow_mut();
                    let ptr = match var_table.get(&e.name) {
                        None => {
                            println!("error: {} not found", e.name());
                            std::process::exit(1);
                        },
                        Some(v) => v
                    };
                    int_cell.set(builder.build_load(*ptr, &e.name()));
                },
                Expr::BinaryOp(e) => {
                    (eval_int_formula.f)(&eval_int_formula, e.left_expr);
                    let left = int_cell.get().into_int_value();
                    (eval_int_formula.f)(&eval_int_formula, e.right_expr);
                    let right = int_cell.get().into_int_value();
                    let mut ret_int_val:IntValue = context.i32_type().const_int(0, false);
                    match e.kind {
                        Kind::Plus => {
                            ret_int_val = builder.build_int_add(left, right, "");
                        },
                        Kind::Minus => {
                            ret_int_val = builder.build_int_sub(left, right, "");
                        },
                        Kind::Multi => {
                            ret_int_val = builder.build_int_mul(left, right, "");
                        },
                        Kind::Divi => {
                            ret_int_val = builder.build_int_unsigned_div(left, right, "");
                        }
                        _ => { std::process::exit(1); }
                    }
                    
                    int_cell.set(BasicValueEnum::IntValue(ret_int_val));
                },
                Expr::Equal(e) => {
                    (eval_int_formula.f)(&eval_int_formula, e.left_expr);
                    let left = int_cell.get().into_int_value();
                    (eval_int_formula.f)(&eval_int_formula, e.right_expr);
                    let right = int_cell.get().into_int_value();
                    let success = builder.build_int_compare(
                        IntPredicate::EQ,
                        left,
                        right,
                        "success",
                    );
                    int_cell.set(BasicValueEnum::IntValue(success));
                }
                _ => {
                    int_cell.set(BasicValueEnum::IntValue(context.i32_type().const_int(0, false)));
                }
            };
        }
    };



    let eval_char = |expr:&Expr|{
        match expr {
            Expr::Char(c) => {
                return BasicValueEnum::IntValue(context.i32_type().const_int(c.eval() as u64, false));
            },
            _ => {
                println!("error: char");
                std::process::exit(1);
            }
        }
    };


    let declare_int = |name: String, val: IntValue| -> PointerValue{
        let i32_type = context.i32_type();
        let int_ref: PointerValue = builder.build_alloca(i32_type, &name);
        let _ = builder.build_store(int_ref, val);
        int_ref
    };

    let emit_global_string = |string: &&str, name: &str|{
        let i8 = context.i8_type();
        let ty = i8.array_type(string.len() as u32);
        let gv = module.add_global(ty, Some(AddressSpace::Generic), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&context.const_string(string.as_ref(), false));

        let pointer_value = builder.build_pointer_cast(
            gv.as_pointer_value(),
            i8.ptr_type(AddressSpace::Generic),
            name,
        );

        pointer_value
    };
    let emit_printf_call = |string: &&str, name: &str|{
        let pointer_value = emit_global_string( string, name);
        let func = module.get_function("puts");
        builder.build_call(func.unwrap(), &[pointer_value.into()], "");

    };
    let print_int = |ptr: PointerValue| {
        let str_ptr = emit_global_string(&"%d\n\0", "");
        let ptr2 = builder.build_load(ptr, "");
        let func = module.get_function("printf");
        builder.build_call(func.unwrap(), &[str_ptr.into(), ptr2.into()], "");
    };

    /* 
    let print_char = |ptr: PointerValue| {
        let func = module.get_function("putchar");
        builder.build_call(func.unwrap(), &[ptr.into()], "");
    };
    */
    let ast_to_llvm = |ast: &Expr| {
        
        match ast {
        Expr::Assign(e) => {
            let left = e.left_expr.clone().name();
            match e.right_expr.clone() {
                Expr::Int(_) | Expr::BinaryOp(_)=> {
                    (eval_int_formula.f)(&eval_int_formula, e.right_expr.clone());
                    let right = int_cell.get();
                    let ptr = declare_int(left.clone(), right.into_int_value());
                    var_table_cell.borrow_mut().insert(left, ptr);
                },
                Expr::Char(_) => {
                    let right = eval_char(&e.right_expr.clone());
                    let ptr = declare_int(left.clone(), right.into_int_value());
                    var_table_cell.borrow_mut().insert(left, ptr);
                },
                Expr::Str(s) => {
                    let string: &str = &s.eval();
                    let ptr = emit_global_string(&string, &left);
                    var_table_cell.borrow_mut().insert(left, ptr);
                },
                _ => {

                }
            }
            
        },
        Expr::Print(e) => {
            let val = e.val.clone();
            match val {
                Expr::Ident(_) => {
                    let var_table = var_table_cell.borrow();
                    let ptr = match var_table.get(&e.val.name()) {
                        None => {println!("error {} not found", e.val.name()); std::process::exit(1); },
                        Some(p) => *p
                    };
                    print_int(ptr);
                }
                Expr::Int(_) | Expr::BinaryOp(_) | Expr::Equal(_)=> {
                    (eval_int_formula.f)(&eval_int_formula, val);
                    let int_val = int_cell.get().into_int_value();
                    let s = int_val.print_to_string().to_string();
                    let vec:Vec<&str> = s.split_whitespace().collect();
                    let s_to_print:&str = &(vec[1].to_string() + "\0");
                    emit_printf_call(&s_to_print, "int");
                },
                Expr::Char(_) => {
                    let char_val = eval_char(&val);
                    let s = get_int_from_int_val(char_val.into_int_value());
                    let fun = module.get_function("putchar");
                    builder.build_call(fun.unwrap(), &[context.i32_type().const_int(s as u64, false).into()], "putchar");
                    builder.build_call(fun.unwrap(), &[context.i32_type().const_int('\n' as u64, false).into()], "putchar");
                },
                Expr::Str(s) => {
                    // emit_global_stringでstringを出力する
                    let string:&str = &s.eval();
                    emit_printf_call(&string, "");
                }
                _ => {
                
                }
            }
        },
        Expr::If(i) => {
            let condition = i.clone().condition;
            
            let list = i.clone().list;

        }
        _ => {
            
        }
    }};

    for node in ast {
        ast_to_llvm(node);
    }
    builder.build_return(Some(&i32_type.const_int(0, false)));

    let _result = module.print_to_file("main.ll");
}

fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Kind {
    Lparen, Rparen, Plus, Minus, Multi, Divi, Equal, NotEq,
    Less, LessEq, Greater, GreaterEq, SngQ, DblQ, Assign, Semicolon,
    If, Else, Print, Ident, Int,
    Str, Letter, Digit, Nulkind, EofTkn, Others, Endlist,
    Lbrace, Rbrace, Char, Nyaan, Addasgn, Mnuasgn, Multiasgn, Divasgn
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub text: String,
    pub chr: char,
    pub kind: Kind,
    pub val: i32,
}
impl Token {
    pub fn new() -> Token {
        Token{text: "".to_string(), chr:' ', kind: Kind::Others, val: 0}
    }
}

pub struct KeyWd<'a> {
    pub val: &'a str,
    pub kind: Kind
}


#[allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone, )]
pub enum Ch {
    Others, Digit, Letter, Assign, Lparen, Rparen, Less, Greater,
    Plus, Minus, Multi, Divi, SngQ, DblQ, Semicolon, Lbrace, Rbrace,
    EOF,Exclam,
}




#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Int(Int),
    BinaryOp(Box<BinaryOp>),
    Ident(Box<Ident>),
    Assign(Box<Assign>),
    Print(Box<Print>),
    Char(Char),
    Str(Str),
    Equal(Box<Equal>),
    If(Box<If>),
    Nope
}
impl Expr {

    fn name(&self) -> String {
        match self 
        {
            Expr::Ident(e) => e.name(),
            Expr::Int(e) => type_of(e),
            Expr::BinaryOp(e) => type_of(e),
            Expr::Assign(e) => type_of(e),
            Expr::Print(e) => type_of(e),
            Expr::Char(e) => type_of(e),
            Expr::Str(e) => type_of(e),
            Expr::Equal(e) => type_of(e),
            Expr::If(e) => type_of(e),
            Expr::Nope => "None".to_string()
            
        }
    }
}


// Int: ����
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Int(i32);
impl Int {
    pub fn new(val: i32) -> Int {
        Int(val)
    }
    fn eval(&self) -> i32 {
        self.0
    }
}

// Char: char
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Char(char);
impl Char {
    pub fn new(val: char) -> Char {
        Char(val)
    }
    fn eval(&self) -> char {
        self.0
    }
}

// String: string
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Str(String);
impl Str {
    pub fn new(val: String) -> Str {
        Str(val)
    }
    fn eval(&self)-> String {
        self.0.clone()
    }
}

// BinaryOp: �l�����Z + - * /
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryOp {
    pub kind: Kind,
    pub left_expr: Expr,
    pub right_expr: Expr
}
impl BinaryOp {
    pub fn new(kind: Kind, left_expr:Expr, right_expr: Expr)-> BinaryOp {
        BinaryOp {kind, left_expr, right_expr}
    }
}

// Ident: 変数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    name: String,
    kind: Kind, 
}
impl Ident {
    pub fn new(name: String, kind: Kind) -> Ident {
        Ident {name, kind}
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

// Assign: =
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assign {
    pub left_expr: Expr,
    pub right_expr: Expr,
}
impl Assign {
    pub fn new(left_expr:Expr, right_expr: Expr) -> Assign {
        Assign {left_expr, right_expr}
    }
}


// Print: print
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Print {
    pub val: Expr,
}
impl Print {
    pub fn new(val: Expr) -> Print {
        Print { val }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Equal {
    pub left_expr: Expr,
    pub right_expr: Expr,
}
impl Equal {
    pub fn new(left_expr: Expr, right_expr: Expr) -> Equal {
        Equal {left_expr, right_expr}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct If {
    pub condition: Expr,
    pub list: Vec<Expr>
}
impl If {
    pub fn new(condition: Expr) -> If {
        let list:Vec<Expr> = Vec::new();
        If { condition, list }
    }
}