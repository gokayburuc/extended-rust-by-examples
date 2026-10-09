#[allow(dead_code)]
#[allow(unused_variables)]
pub fn execute_variable_bindings() {
    let an_integer: u32 = 1u32;
    let a_boolean: bool = true;

    // TODO: unit type
    let an_unit: () = ();

    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {:?}", an_unit);

    // NOTE: _ underscore removes the linter message
    let _unused_variable = 3u32;

    // WARN: without underscore gives lint error
    let noisy_unused_variable = 2u32;
}
