// INFO: return değeri Some ya da None olarak dönecektir.
fn get_age(username: &str) -> Option<u32> {
    if username == "Ahmet" { Some(25) } else { None }
}

pub fn execute_option() {
    let result = get_age("Mehmet");

    match result {
        Some(age) => println!("Kullanıcı yaşı: {}", age),
        None => println!("There is no this username"),
    }
}
