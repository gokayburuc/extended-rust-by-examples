use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

pub fn execute_into() {
    let int = 5;

    let num: Number = int.into();
    println!("My number is {:?}", num);
}
