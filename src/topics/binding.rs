fn age() -> u32 {
    15
}

#[allow(unreachable_patterns)]
pub fn execute_binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I havent celebrated my first birthday yet"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),

        // WARN: this is unreachable
        n @ (1 | 7 | 15 | 13) => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

pub fn execute_enum_binding() {
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
