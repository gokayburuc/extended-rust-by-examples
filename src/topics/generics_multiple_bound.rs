use std::fmt::{Debug, Display};

// generics bound function
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

// multiple generic bounded function
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

pub fn execute_generic_multiple_bound() {
    let sample_string = "words";
    let sample_array = [1, 2, 3];
    let sample_vec = vec![1, 2, 3];

    compare_prints(&sample_string);
    compare_types(&sample_array, &sample_vec);
}
