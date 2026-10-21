use crate::additional::{
    traits_default_methods::execute_default_method_trait,
    traits_join_function::{CatAgain, report},
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
}
