use crate::{
    additional::{
        traits_default_methods::execute_default_method_trait,
        traits_join_function::{CatAgain, report},
    },
    topics::generics_multiple_bound::execute_generic_multiple_bound,
};

mod additional;
mod smallproject;
mod topics;

fn main() {
    execute_default_method_trait();
    let our_cat = CatAgain {
        cat_name: String::from("Isot"),
    };
    report(&our_cat);

    // topics::generics_multiple_bound::execute_generic_multiple_bound();
    execute_generic_multiple_bound();
}
