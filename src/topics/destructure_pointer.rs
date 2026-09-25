// `reference` adlı bir referans oluşturuluyor ve 4 değerine işaret ediyor.
pub fn execute_destructure_pointer() {
    let reference = &4;

    // `reference` referansını desektif ederek `val` değerini alıyoruz.
    match reference {
        &val => println!("got a value via destruring: {:?}", val),
    }

    // `reference` referansını dereference ederek doğrudan `val` değerini alıyoruz.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // `not_a_reference` adlı bir değişken oluşturuluyor, referans değil.
    let _not_a_reference = 3;
    // `is_a_reference` adlı bir referans oluşturuluyor ve 3 değerine işaret ediyor.
    let ref _is_a_reference = 3;

    // `value` adlı bir değişken oluşturuluyor ve 5 değerine sahip.
    let value = 5;
    // `mut_value` adlı bir değişken oluşturuluyor ve 6 değerine sahip, mutable (değiştirilebilir).
    let mut mut_value = 6;

    // `value` değerine bir referans alınıyor ve `r` adlı bir referans olarak kullanılıyor.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // `mut_value` değerine bir mutable referans alınıyor, 10 ekleniyor ve `m` olarak kullanılıyor.
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. mut_value: {:?}", m);
        }
    }
}
