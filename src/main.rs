use crate::topics::{high_order_functions, iterator_any, iterator_search};

mod additional;
mod topics;

fn main() {
    // closure_type_anonymity::execute_closure_type_anonymity();
    // closure_input_functions::execute_closure_input_function();
    // closure_as_output_parameter::execute_closure_output_parameter();
    iterator_any::execute_iterator_any();
    iterator_search::execute_iterator_search();
    iterator_search::execute_iterator_position();
    high_order_functions::execute_high_order_function();
}
