use inkwell::{context::Context};
use inkwell::builder::Builder;
use inkwell::{AddressSpace};
use inkwell::module::{Linkage, Module};
use inkwell::values::{IntValue, PointerValue, BasicValueEnum};
use inkwell::OptimizationLevel;
use std::collections::HashMap;
pub mod parser;
pub mod tokenizer;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    expr_list: &'a Vec<Expr>,
    var_table: &'a HashMap<String, PointerValue<'a>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'ctx Context, builder: &'a Builder<'ctx>, module: &'a Module<'ctx>, expr_list: &'a Vec<Expr>, var_table: &'a HashMap<String, PointerValue<'a>>) {
        let compiler = Compiler {
            context,
            builder,
            module,
            expr_list,
            var_table
        };

        compiler.compile();
    }

    pub fn compile(&self) {
        let i32_type = self.context.i32_type();
        let function_type = i32_type.fn_type(&[], false);

        let function = self.module.add_function("main", function_type, None);
        let basic_block = self.context.append_basic_block(function, "entrypoint");
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);
        let putchar_type = i32_type.fn_type(&[i32_type.into()], false);

        self.module.add_function("puts", printf_type, Some(Linkage::External));
        self.module.add_function("putchar", putchar_type, None);
        self.builder.position_at_end(basic_block);

        //let i32_type = self.emit_printf_call(&"hello, world!\n", "hello");
        self.llvm();
        self.builder.build_return(Some(&i32_type.const_int(0, false)));

        let _result = self.module.print_to_file("main.ll");
        self.execute()
    }
    fn llvm(&self) {
        for expr in self.expr_list {
            self.ast_to_llvm(expr);
        }
    }
    fn ast_to_llvm(&self, ast: &Expr) {
        match ast {
            Expr::Assign(e) => {
                let left = e.left_expr.clone().name();
                match e.left_expr.clone() {
                    Expr::Int(i) => {
                        let right = self.eval_int_formula(e.right_expr.clone());
                        self.declare_int(left, right.into_int_value());
                    },
                    Expr::Char(c) => {
                        let right = self.eval_char(e.right_expr.clone());
                        self.declare_int(left, right.into_int_value());
                    },
                    Expr::Str(s) => {
                        let string: &str = &s.eval();
                        let right = self.emit_global_string(&string, &left);
                        // self.var_table.insert(left, right);
                    }
                    _ => {

                    }
                }

            },
            Expr::Print(e) => {
                let val = e.val.clone();
                match val {
                    Expr::Ident(_) => {

                    }
                    Expr::Int(_) | Expr::BinaryOp(_) => {
                        let int_val = self.eval_int_formula(val).into_int_value();
                        let s = int_val.print_to_string().to_string();
                        let vec:Vec<&str> = s.split_whitespace().collect();
                        let s_to_print:&str = &(vec[1].to_string() + "\0");
                        self.emit_printf_call(&s_to_print, "int");
                    },
                    Expr::Char(_) => {
                        let char_val = self.eval_char(val).into_int_value();
                        let s = self.get_int_from_int_value(char_val);
                        let fun = self.module.get_function("putchar");
                        self.builder.build_call(fun.unwrap(), &[self.context.i32_type().const_int(s as u64, false).into()], "putchar");
                        self.builder.build_call(fun.unwrap(), &[self.context.i32_type().const_int('\n' as u64, false).into()], "putchar");
                    },
                    Expr::Str(s) => {
                        // emit_global_stringでstringを出力する
                        let string:&str = &s.eval();
                        self.emit_printf_call(&string, "");
                    }
                    _ => {
                    
                    }
                }
            },
            _ => {
                
            }
        }
    }
    #[allow(unused_mut)]
    fn eval_int_formula(&self, expr: Expr) -> BasicValueEnum {
        match expr {
            Expr::Int(e) => {
                return BasicValueEnum::IntValue(self.context.i32_type().const_int(e.eval() as u64, false));
            },
            Expr::Ident(e) => {
                println!("called {}", e.name);
                let ptr = self.var_table.get(&e.name()).unwrap();
                return self.builder.build_load(*ptr, &e.name());
                
            },
            Expr::BinaryOp(e) => {
                let left = self.eval_int_formula(e.left_expr).into_int_value();
                let right = self.eval_int_formula(e.right_expr).into_int_value();
                let mut ret_int_val: IntValue;
                match e.kind {
                    Kind::Plus => {
                        ret_int_val = self.builder.build_int_add(left, right, "");
                    },
                    Kind::Minus => {
                        ret_int_val = self.builder.build_int_sub(left, right, "");
                    },
                    Kind::Multi => {
                        ret_int_val = self.builder.build_int_mul(left, right, "");
                    },
                    Kind::Divi => {
                        ret_int_val = self.builder.build_int_unsigned_div(left, right, "");
                    }
                    _ => {std::process::exit(1);}
                }
                return BasicValueEnum::IntValue(ret_int_val);
            }
            _ => {
                return BasicValueEnum::IntValue(self.context.i32_type().const_int(0, false));
            }
        }
    }
    fn eval_char(&self, expr: Expr) -> BasicValueEnum {
        match expr {
            Expr::Char(c) => {
                return BasicValueEnum::IntValue(self.context.i8_type().const_int(c.eval() as u64, false));
            },
            _ => {
                println!("error: char");
                std::process::exit(1);
            }
        }
    }
    fn emit_printf_call(&self, hello_str: &&str, name: &str) {
        let pointer_value = self.emit_global_string(hello_str, name);
        let func = self.module.get_function("puts");
        self.builder.build_call(func.unwrap(), &[pointer_value.into()], "");

    }

    fn execute(&self) {
        let ee = self.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        let maybe_fn = unsafe {
            ee.get_function::<unsafe extern "C" fn() -> f64>("main")
        };

        let compiled_fn = match maybe_fn {
            Ok(f) => f,
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        unsafe {
            compiled_fn.call();
        }
    }

    fn emit_global_string(&self, string: &&str, name: &str) -> PointerValue {
        let ty = self.context.i8_type().array_type(string.len() as u32);
        let gv = self.module.add_global(ty, Some(AddressSpace::Generic), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&self.context.const_string(string.as_ref(), false));

        let pointer_value = self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        );

        pointer_value
    }
    fn declare_int(&self, name: String, val: IntValue) {
        let i32_type = self.context.i32_type();
        let int_ref: PointerValue = self.builder.build_alloca(i32_type, &name);
        let _ = self.builder.build_store(int_ref, val);
    }

    fn declare_char(&self, name:String, chr: IntValue) {
        let i8_type = self.context.i8_type();
        let char_ref: PointerValue = self.builder.build_alloca(i8_type, &name);
        let _ = self.builder.build_store(char_ref, chr);
    }

    fn get_int_from_int_value(&self, int_val: IntValue) -> i32{
        let s = int_val.print_to_string().to_string();
        let vec:Vec<&str> = s.split_whitespace().collect();
        let val: i32 = vec[1].parse().unwrap();
        val
    }
}  

pub fn create_compiler(expr_list: &Vec<Expr>) {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();
    let var_table: HashMap<String, PointerValue> = HashMap::new();
    Compiler::new(&context, &builder, &module, expr_list, &var_table);
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
    Nope
}
impl Expr {

    fn name(&self) -> String {
        match self {
            Expr::Ident(e) => e.name(),
            Expr::Int(e) => type_of(e),
            Expr::BinaryOp(e) => type_of(e),
            Expr::Assign(e) => type_of(e),
            Expr::Print(e) => type_of(e),
            Expr::Char(e) => type_of(e),
            Expr::Str(e) => type_of(e),
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
    fn eval(&self) -> String {
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

