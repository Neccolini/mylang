mod parser;

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::{AddressSpace};
use inkwell::module::{Linkage, Module};
use inkwell::types::IntType;
use inkwell::values::PointerValue;


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


fn main() {

}