pub fn expressions_execute() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        println!("x square: {}", x_squared);
        println!("x cube: {}", x_cubed);
    };

    let z = { 2 * x };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
