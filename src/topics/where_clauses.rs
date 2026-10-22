use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// INFO: impl for where syntax
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self))
    }
}

// TODO: how to access print_in_option directly
// INFO: this print_in_option method can be accessable because of the generics and traits
fn execute_where_clause() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}
