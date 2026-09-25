pub fn execute_destructure_tuple() {
    let triple_tuple = (0, -2, 3);

    println!("Tell me about {:?}", triple_tuple);

    match triple_tuple {
        (0, y, z) => println!("First is 0, y is {:?} and z is {:?}", y, z),
        (1, ..) => println!("First is 1 and rest is doesnt matter"),
        (.., 2) => println!("Last is 2 and rest is doesnt matter"),
        (3, .., 4) => println!("First is 3, last is 4, and rest is doesnt matter"),
        _ => println!("It doesnt matter what they are"),
    }
}
