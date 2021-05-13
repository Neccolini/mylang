use inkwell::{context::Context, execution_engine::ExecutionEngine};
use inkwell::builder::Builder;
use inkwell::{AddressSpace};
use inkwell::module::{Linkage, Module};
use inkwell::types::IntType;
use inkwell::values::{IntValue, PointerValue};
use std::collections::HashMap;
pub mod parser;
pub mod tokenizer;
#[allow(dead_code)]
struct Llvm<'c> {
    context: &'c Context,
    builder: Builder<'c>,
    module: Module<'c>,
    int_var_table: &'c mut HashMap<String, PointerValue<'c>>,
}

impl<'c> Llvm<'c> {
    pub fn new(context: &'c Context, module: Module<'c>, builder: Builder<'c>, int_var_table: &'c mut HashMap<String, PointerValue<'c>>) -> Llvm<'c> {

        Llvm {context, module, builder, int_var_table }

    }

    // for print
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
    fn llvm(&'static self, expr_list: &Vec<Expr>) {
        for expr in expr_list {
            self.expr_to_llvm_element(expr);
        }
    }
    fn expr_to_llvm_element(&'static self, expr: &Expr) -> Data {
        match expr {
            Expr::Assign(e) => {
                let left = &e.left_expr.clone();
                let right = &e.right_expr.clone();
                let leftname = left.name(); //Identのはず
                let right_data = self.eval_int_formula(right.clone(), false).eval();
                return self.declare_int(leftname, &right_data);
            },
            Expr::Print(e) => {
                let val = e.val.clone();
                std::process::exit(1);
            }
            _ => {
                std::process::exit(1);
            }
        }
    }
    fn eval_int_formula(&'static self, expr: Expr, is_left: bool) -> Data {
        match expr {
            Expr::Int(e) => {
                return self.const_int(e.eval());
            }
            Expr::BinaryOp(e) => {
                match e.kind {
                    Kind::Plus => {
                        let left = self.eval_int_formula(e.left_expr, true).eval();
                        let right = self.eval_int_formula(e.right_expr, false).eval();
                        let add_pos = self.builder.build_int_add(left, right, "");
                        return Data::Int(add_pos);
                    },
                    Kind::Minus => {
                        let left = self.eval_int_formula(e.left_expr, true).eval();
                        let right = self.eval_int_formula(e.right_expr, false).eval();
                        let sub_pos = self.builder.build_int_sub(left, right, "");
                        return Data::Int(sub_pos);
                    },
                    Kind::Multi => {
                        let left = self.eval_int_formula(e.left_expr, true).eval();
                        let right = self.eval_int_formula(e.right_expr, false).eval();
                        let mul_pos = self.builder.build_int_mul(left, right, "");
                        return Data::Int(mul_pos);
                    },
                    Kind::Divi => {
                        let left = self.eval_int_formula(e.left_expr, true).eval();
                        let right = self.eval_int_formula(e.right_expr, false).eval();
                        let div_pos = self.builder.build_int_signed_div(left, right, "");
                        return Data::Int(div_pos);
                    },
                    _ => {std::process::exit(1); }
                }
            }, 
            _ => {
                std::process::exit(1);
            }
        }
    }
    fn load_int(&self, name: String) -> PointerValue {
        let data = *self.int_var_table.get(&name).unwrap(); // todo: match でerror処理すべき
        // build_loadして、取り出す命令を書く
        self.builder.build_load(data, &name); // todo now
        // let int_var = Data::IntVar(Box::new(IntVar::new(data, )));
        return data;
    }

    fn declare_int(&'static self, name: String,  i32_value:  &'static IntValue) -> Data {
        let i32_type = self.context.i32_type();
        let const_int_ref:PointerValue = self.builder.build_alloca(i32_type, &name);
        let _ = self.builder.build_store(const_int_ref, *i32_value);
        let int_var = IntVar::new(const_int_ref, *i32_value);
        let data = Data::IntVar(Box::new(IntVar::new(const_int_ref, *i32_value)));
        self.int_var_table.insert(name, const_int_ref);
        return data;
    }

    fn const_int(&'static self, val: i32) -> Data<'static> {
        Data::Int(self.context.i32_type().const_int(val as u64, false))
    }
}
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
  }
pub fn create_llvm(expr_list: &Vec<Expr>) {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let mut variable_table:HashMap<String, PointerValue> = HashMap::new();
    let llvm = Llvm::new(&context, module, builder,  &mut variable_table);
    llvm.llvm(expr_list);
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
    fn name(&self) -> String {
        match self {
            Expr::Ident(e) => e.name,
            Expr::Int(e) => type_of(e),
            Expr::BinaryOp(e) => type_of(e),
            Expr::Assign(e) => type_of(e),
            Expr::Print(e) => type_of(e),
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
    fn eval(&self) -> i32 {
        0
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn kind(&self) -> Kind {
        self.kind
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

enum Data<'c> {
    //変数
    IntVar(Box<IntVar<'c>>),
    //即値
    Int(IntValue<'c>),
    String,
    Char,
    Float,
    Bool,
    None
}

impl Data<'_> {
    fn eval(&'static self) -> IntValue {
        match self {
            Data::IntVar(e) => e.eval(),
            Data::Int(e) => *e,
            default => {std::process::exit(1);}
        }
    }
}
struct IntVar<'c> {
    ptr: PointerValue<'c>,
    value: IntValue<'c>
}
impl IntVar<'static> {
    fn new(ptr: PointerValue<'static>, value: IntValue<'static>) -> IntVar<'static> {
        IntVar {ptr, value}
    }
    fn ptr(&self) -> PointerValue {
        self.ptr
    }
    fn eval(&self) -> IntValue {
        self.value
    }
}

enum Variable<'c> {
    IntVar(Box<IntVar<'c>>)
}

/*
enum Immediate {
    Int,
    String,
    Char,
    Float,
    Bool,
}

*/