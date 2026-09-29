pub fn execute_closure() {
    // Dış değişkeni tanımlıyoruz, değerini 42 olarak atıyoruz.
    let outer_var = 42;

    // Tipi açıkça belirtilmiş bir closure: `i32` parametresi alır ve `i32` döndürür.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // Tipi çıkarımlı bir closure: Rust, parametre ve dönüş tipini kendisi belirler.
    let closure_inferred = |i| i + outer_var;

    // Anotasyonlu closure'ı çağırarak sonucu yazdırıyoruz.
    println!("Closure annotated: {}", closure_annotated(1));
    // Çıkarımlı closure'ı çağırarak sonucu yazdırıyoruz.
    println!("Closure inferred: {}", closure_inferred(1));

    // Hiç parametre almayan ve her zaman 1 döndüren bir closure.
    let one = || 1;
    // Bu closure'ı çağırarak sonucu yazdırıyoruz.
    println!("Closure Returning one: {}", one());
}

/// x sayısına 5 ekliyor ve ekrana yazdırıyor
pub fn execute_closure_capture(x: i32) {
    // let x = 10;
    let ekle = |y| x + y;

    println!("Sonuç: {}", ekle(5));
}

// FIX: fix the function
pub fn execute_closure_type_inference(a: i32, b: i32) -> () {
    let cikar = |a, b| a - b;
    // FIX: fix diagnostics errors
    return cikar; // FIX: unittype ()
}

// TODO: add line comments and explanations for this function
fn operation_make<F: Fn(i32) -> i32>(f: F, deger: i32) -> i32 {
    f(deger)
}

fn execute_closure_square() {
    let square = |x| x * x;
    println!("Kare: {}", operation_make(square, 4));
}
