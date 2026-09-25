#![allow(unreachable_code, unused_labels)]
pub fn execute_nested_loop() {
    // NOTE:  usage of label
    'outer: loop {
        println!("Entered the outer loop");

        // NOTE:  another label
        'inner: loop {
            println!("Entered the inner loop");

            // WARN: 'label must be passed to the break / continue statement
            break 'outer;
        }

        println!("This ppoint will never be reached");
    }

    println!("Exited the outer loop");
}
