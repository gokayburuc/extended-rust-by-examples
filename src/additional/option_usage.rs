// ============================================================
// KONU     : `Option<T>` Kullanımı
// KAYNAK   : https://doc.rust-lang.org/rust-by-example/std/option.html
// AÇIKLAMA : Değerin mevcut (`Some`) veya yokluğunu (`None`) temsil eden
//            `Option<T>` enum'ının `match`, `if let` ve metotlarla
//            kullanımını açıklar.
// ============================================================

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
