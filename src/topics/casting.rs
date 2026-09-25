#![allow(overflowing_literals)]
#[allow(dead_code)]
pub fn execute_casting() {
    // TODO: 65.4321_f32 in rust
    let decimal = 65.4321_f32;

    // WARN: this will give an error
    // let integer: u8 = decimal;

    // Explicit conversion using casting
    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 - (256*3) = 232;
    println!("1000 as u16 is: {}", 1000 as u16);

    // WARN: literal out of range error
    println!("1000 as a u8 is: {}", 1000 as u8);

    // -1 + 256 = 255;
    println!("-1 as u8 is: {}", (-1i8) as u8);

    println!("128 as i16 is: {}", 128 as i16);

    // WARN: literal out of range error
    println!(" 128 as a i8 is : {}", 128 as i8);

    // WARN: literal out of range error
    println!("1000 as a u8 is: {}", 1000 as u8);

    // WARN: literal out of range error
    println!("232 as a i8 is: {}", 232 as i8);

    // 300.0_f32 is float 32 300.0
    println!("300.0 as u8 is : {}", 300.0_f32 as u8);

    // WARN: nan as u8 is 0
    println!("nan as u8 is: {}", f32::NAN as u8);

    unsafe {
        // 300.0 as u8 is 44  -> 300 - 256 = 44;
        println!(
            "unsafe 300.0 as u8 is: {}",
            300.0_f32.to_int_unchecked::<u8>()
        );

        println!(
            //
            "unsafe -100.0 as u8 is: {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );

        println!("unsafe nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
