// use create for shorthand usage
use crate::topics::{closures, methods};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    methods::execute_method();
    closures::execute_closures();
}
