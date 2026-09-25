#[allow(dead_code)]
pub fn execute_match() {
    let number: i32 = 14;

    println!("Tell me about number: {}", number);

    match number {
        1 => println!("One!"),

        // optional matches with or | symbol
        2 | 3 | 4 | 5 | 6 => println!("Optimus Prime!"),

        //  match in range
        7..=20 => println!("Autobot!"),

        // default value
        _ => println!("No Special"),
    }

    let sample_boolean = true;

    let binary_match = match sample_boolean {
        false => 0,
        true => 1,
    };

    println!("{}  ->  {} ", sample_boolean, binary_match);
}
