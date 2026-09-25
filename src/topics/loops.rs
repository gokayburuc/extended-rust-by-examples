pub fn loop_execute() {
    let mut count = 0u32;

    println!("Count until infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // skips the rest of this iteration
            continue;
        }

        println! {"{}", count};

        if count == 5 {
            println!("OK, thats enough");
            break;
        }
    }
}
