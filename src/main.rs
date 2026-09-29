// use create for shorthand usage
use crate::topics::{capturing_closures, closures, methods};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    capturing_closures::execute_capturing_closures();
    capturing_closures::execute_capturing_move();
    closures::execute_closures();
    methods::execute_method();
}
