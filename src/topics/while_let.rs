#[allow(dead_code)]
pub fn execute_while_let() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
        } else {
            println!("i is {:?}. Try again", i);
            optional = Some(i + 1);
            // increase value in each action
        }
    }
}
