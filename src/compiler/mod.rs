use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::{AddressSpace};
use inkwell::module::{Linkage, Module};
use inkwell::types::IntType;
use inkwell::values::PointerValue;
pub mod parser;
pub mod tokenizer;

pub struct Llvm<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}
impl<'a, 'ctx> Llvm<'a, 'ctx> {
    pub fn new(context: &'ctx Context, module: &'a Module<'ctx>, builder: &'a Builder<'ctx>) -> Llvm<'a, 'ctx> {
        Llvm {context, module, builder}
    }
    fn emit_global_string(&self, str: &&str, name: &str) -> PointerValue {
        let ty = self.context.i8_type().array_type(str.len() as u32);
        let gv = self.module.add_global(ty, Some(AddressSpace::Generic), name);
        gv.set_linkage(Linkage::Internal);
        gv.set_initializer(&self.context.const_string(str.as_ref(), false));
        
        self.builder.build_pointer_cast(
            gv.as_pointer_value(),
            self.context.i8_type().ptr_type(AddressSpace::Generic),
            name,
        )
    }
    fn emit_printf_call(&self, str: &&str, name: &str) -> IntType {
        let i32_type = self.context.i32_type();
        let str_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[str_type.into()], true);

        let printf = self.module.add_function("puts", printf_type, Some(Linkage::External));
        let pointer_value = self.emit_global_string(str, name);
        self.builder.build_call(printf, &[pointer_value.into()], "");

        i32_type
    }
    fn expr_to_llvm(&self, expr_list: &Vec<Expr>) {
        for expr in expr_list {
            match expr {
                Expr::Assign(e) => {
                    
                },
                Expr::Print(e) => {
                    let x = e.val.clone();
                    
                },
                _ => {
    
                }
            }
        }
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Kind {
    Lparen, Rparen, Plus, Minus, Multi, Divi, Equal, NotEq,
    Less, LessEq, Greater, GreaterEq, SngQ, DblQ, Assign, Semicolon,
    If, Else, Print, Ident, Int,
    String, Letter, Digit, Nulkind, EofTkn, Others, Endlist,
    Lbrace, Rbrace, Char, Nyaan, Addasgn, Mnuasgn, Multiasgn, Divasgn
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token{
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
    Nope
}
impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Int(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval(),
            Expr::Ident(e) => e.eval(),
            Expr::Assign(e) => e.eval(),
            Expr::Print(e) => e.eval(),
            Expr::Nope => 0
        }
    }
}

// Int: ????
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

// BinaryOp: ?l?????Z + - * /
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

// Ident: ???
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    pub kind: Kind, 
}
impl Ident {
    pub fn new(name: String, kind: Kind) -> Ident {
        Ident {name, kind}
    }
    pub fn eval(&self) -> i32 {
        0
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
    pub fn eval(&self) -> i32 {
        0
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
    pub fn eval(&self) -> i32 {
        self.val.eval()
    }
}