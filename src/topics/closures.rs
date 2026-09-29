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
    let ekle = |y| x + y;
    println!("Sonuç: {}", ekle(5));
}

/// FIX: Fixed the function to avoid shadowing and return the result of the closure.
pub fn execute_closure_type_inference(a: i32, b: i32) -> i32 {
    let cikar = |x, y| x - y;
    cikar(a, b) // Return the result of calling the closure.
}

/// TODO: Added line comments and explanations for this function.
/// This function takes a closure `f` and an integer `deger`, applies the closure to `deger`, and returns the result.
fn operation_make<F: Fn(i32) -> i32>(f: F, deger: i32) -> i32 {
    f(deger)
}

/// This function defines a closure to square a number and uses `operation_make` to apply it.
fn execute_closure_square() {
    let square = |x| x * x;
    println!("Kare: {}", operation_make(square, 4));
}

/// Example function demonstrating closures with outer variables.
pub fn execute_closures() {
    let outer_var = 42;

    // INFO: adds +1 to given i value
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i: i32| i + outer_var;

    println!("closure_annotated:{}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // WARN: this works like an anonymous function
    let one = || 1;
    println!("closure returning:{}", one());
}
