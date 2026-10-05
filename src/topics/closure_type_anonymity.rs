// `F` must be generic.
fn apply_type_anonymity<F>(f: F)
where
    // INFO:  F only can be used once
    F: FnOnce(),
{
    f();
}

pub fn execute_closure_type_anonymity() {
    let x: i32 = 7;

    // TODO:  why impl Fn() type used here for my_print?
    let my_print = || println!("{}", x);

    apply_type_anonymity(my_print);
}

// TODO: generics <T>
