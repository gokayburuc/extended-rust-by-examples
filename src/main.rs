// use create for shorthand usage
use crate::topics::{module_struct, modules};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    modules::execute_modules();
    module_struct::execute_module_struct();
}
