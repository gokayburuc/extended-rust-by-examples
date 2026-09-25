// Suffix type names are updated as the list given below
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

pub fn execute_aliasing() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nano seconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

