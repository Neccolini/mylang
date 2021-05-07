use inkwell::context::Context;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
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

    // call i32 @putchar(i32 72)
    let fun = module.get_function("putchar");
    builder.build_call(fun.unwrap(), &[i32_type.const_int(72, false).into()], "putchar");
    builder.build_call(fun.unwrap(), &[i32_type.const_int(105, false).into()], "putchar");

    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)));

    module.print_to_stderr();

    Ok(())
}