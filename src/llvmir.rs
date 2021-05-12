use inkwell::context::Context;
use inkwell::builder::Builder;
use std::error::Error;
use inkwell::{AddressSpace};
use inkwell::module::{Linkage, Module};

mod parser;
use parser::{Expr};


pub struct Llvmir<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}
impl<'a, 'ctx> Llvmir<'a, 'ctx> {
    pub fn new(name: &str) -> Llvmir {
        let context = Context::create();
        let module = context.create_module(name);
        let builder = context.create_builder();
        LLvmir {context, module, builder}
    }
}
#[allow(bare_trait_objects)]
#[allow(unused_must_use)]
fn main() -> Result<(), Box<Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    // declare i32 @putchar(i32)
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);
    let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
    let printf_type = i32_type.fn_type(&[str_type.into()], true);
    let printf = module.add_function("puts", printf_type, Some(Linkage::External));
    // define i32 @main() {
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);


    let int_32_type = context.i32_type();
    let i32_value = int_32_type.const_int(10, false);
    let str_i32 = &"10";
    let ty = context.i8_type().array_type(str_i32.len() as u32);
    let gv = module.add_global(ty, Some(AddressSpace::Generic), "s");
    gv.set_linkage(Linkage::Internal);
    gv.set_initializer(&context.const_string(str_i32.as_ref(), false));
    let pointer_value = builder.build_pointer_cast(
        gv.as_pointer_value(),
        context.i8_type().ptr_type(AddressSpace::Generic),
        "s",
    );
    let const_int_ref= builder.build_alloca(int_32_type, "int_value");
    let _ = builder.build_store(const_int_ref, i32_value);
    
    builder.build_call(printf, &[pointer_value.into()], "");
    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)));
    module.print_to_file("test.ll");

    Ok(())
}
/*
fn Printer(context: &Context, str: &str, )

pub fn llvm(expr_list: &Vec<Expr>) {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    // declare i32 @putchar(i32)
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    // define i32 @main() {
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    for expr in expr_list {
        match expr {
            Expr::Ident => {
            },
            Expr::Print => {

            }
        }
    }

}
pub fn generate(
    root: ast::Root


    // println!("root -> \n{:#?}", root);
    for node in root.node {
        // println!("node -> {:#?}", node);
        match node {
            ast::Types::Exp(val) => {
                println!("Matched Type::Exp");

                if val.token == -6 {
                    let text =  match val.value {
                        ast::ValueTypes::Str(val) => val,
                        _ => String::new()
                    };

                    let fun = module.get_function("putchar");
                    for c in text.chars() {
                        let ascii = c.to_string().as_bytes()[0] as u64;
                        builder.build_call(fun.unwrap(), &[i32_type.const_int(ascii, false).into()], "putchar");
                    }
                }
            },
            _ => println!("Unknown type matched")
        }
    }

    builder.build_return(Some(&i32_type.const_int(0, false)));
    module.print_to_stderr();

    // Any option (test)
    Some(ast::Types::Exp(ast::Expression::new(0)))
}
fn main() {

}
*/