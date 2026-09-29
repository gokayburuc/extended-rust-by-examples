// use create for shorthand usage
use crate::topics::{capturing_closures, closures, methods};

mod additional;
// the mods that we are going to use
mod topics;
fn main() {
    methods::execute_methods();
    closures::execute_closure();
    closures::execute_closure_capture(18);
    capturing_closures::execute_capturing_closures();
}
