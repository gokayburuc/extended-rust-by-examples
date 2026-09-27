// use create for shorthand usage
use crate::topics::{
    destructure_enums, destructure_tuples, functions_basic, iflet, let_else, while_let,
};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    destructure_tuples::execute_destructure_tuple();
    destructure_enums::execute_destructure_enums();
    iflet::execute_iflet();
    iflet::execute_if_let_second();
    let_else::execute_get_count_item();
    while_let::execute_while_let();
    functions_basic::execute_function_basic();
}
