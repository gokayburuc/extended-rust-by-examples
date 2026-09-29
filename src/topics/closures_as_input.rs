pub fn execute_closures_apply<F>(f: F)
where
    // TODO: check FnOnce trait
    F: FnOnce(),
{
    f();
}

// TODO: add line comments here
pub fn execute_apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

// TODO: check the function
pub fn execute_closure_as_input() {
    // TODO: why whe imported std::mem here instead of script header?
    use std::mem;
    let greeting = "hello";

    // TODO: check .to_owned() method
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now i can sleep, zzzz");

        // TODO: why we dropped the farewell here from memory ?
        mem::drop(farewell);
    };

    execute_closures_apply(diary);
    // multiple by 2 the x value
    let double = |x| 2 * x;

    println!("3 doubled: {}", execute_apply_to_3(double));
}

// TODO: check ownership
// TODO: check closures
// TODO: check traits
