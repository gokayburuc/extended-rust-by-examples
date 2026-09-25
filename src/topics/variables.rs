#[allow(dead_code)]
pub fn variable_execute() {
    // int
    let my_first_number: i32 = 5;
    println!("{}", my_first_number);

    // mutable int
    let mut my_second_number: i32 = 25;
    println!("{}", my_second_number);
    my_second_number = 45;
    println!("{}", my_second_number);

    // float
    let weather_degree: f32 = 34.45;
    println!("Weather is {} degree", weather_degree);

    // boolean
    let drive_licence: bool = true;
    let engineering_diplom: bool = true;
    println!("Has Licence: {}", drive_licence);
    println!("Has Diplom: {}", engineering_diplom);

    // string
    let firstname: String = String::from("Gokay");
    let lastname: String = String::from("BURUC");
    println!("{} {}", firstname, lastname);
}
