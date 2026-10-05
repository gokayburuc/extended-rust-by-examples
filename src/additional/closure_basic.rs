pub fn execute_closure_square() {
    // | parameter |
    let square = |x| x * x;
    println!("{}", square(5));
}

pub fn execute_closure_sum() {
    let addition = |a, b| a + b;
    println!("{}", addition(10, 20));
}

pub fn execute_normal_addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn execute_closure_string() {
    let first_name = String::from("Gökay");

    let greetings = || {
        println!("Merhaba {}", first_name);
    };

    greetings();

    // WARN:  first_name still available
    println!("{}", first_name);
}

pub fn execute_closure_counter() {
    let mut sayac = 0;

    let mut increase = || {
        sayac += 1;
    };

    increase();
    increase();
    increase();

    println!("{}", sayac);
}

pub fn execute_closure_squre_type() {
    let square = |x: i32| -> i32 { x * x };
    println!("{}", square(5));
}

pub fn execute_closure_multiple_type() {
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("{}", multiply(5, 4));
}
