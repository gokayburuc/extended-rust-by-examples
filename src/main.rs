// use create for shorthand usage

use crate::topics::{destructure_enums, destructure_tuples};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    destructure_tuples::execute_destructure_tuple();
    destructure_enums::execute_destructure_enums();
}
