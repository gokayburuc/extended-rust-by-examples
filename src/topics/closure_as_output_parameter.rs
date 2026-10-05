fn create_a_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    // WARN: not ends with ;
    move || println!("This is a: {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    // WARN: not ends with ;
    move || println!("This is a: {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    // WARN: not ends with ;
    move || println!("This is a: {}", text)
}

pub fn execute_closure_output_parameter() {
    let fn_plain = create_a_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    fn_plain();
    fn_mut();
    fn_once();
}
