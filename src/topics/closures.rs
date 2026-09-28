pub fn execute_closures() {
    let outer_var = 42;

    // INFO: adds +1 to given i value
    // TODO: check the apply method in python
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i: i32| i + outer_var;

    println!("closure_annotated:{}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // WARN:this works like anonymous method function
    let one = || 1;
    println!("closure returning:{}", one());
}
