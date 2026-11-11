// ============================================================
// KONU     : Generics Temelleri (Açıklamalı)
// KAYNAK   : https://doc.rust-lang.org/rust-by-example/generics.html
// AÇIKLAMA : Generic tür parametrelerinin neden kullanıldığını ve temel
//            söz dizimini Türkçe açıklamalarla pekiştiren ek örnektir.
// ============================================================

use std::cmp::PartialOrd;
// fn en_buyuk_i32(a: i32, b: i32) -> i32 {
//     if a > b { a } else { b }
// }
//
// fn en_buyuk_f64(a: f64, b: f64) -> f64 {
//     if a > b { a } else { b }
// }
//
// fn en_buyuk_char(a: char, b: char) -> char {
//     if a > b { a } else { b }
// }

// INFO:  float, number, character için 3 farklı fonksiyon yazmak
// yerine tek bir fonksiyon yazdık

fn maximum<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn execute_function_generics() {
    let number = maximum(3, 7);
    let float_number = maximum(2.5, 1.8);
    let character = maximum('a', 'z');

    println!("{number}, {float_number}, {character}");
}

#[derive(Debug)]
struct MyBox<T> {
    box_content: T,
}

impl<T> MyBox<T> {
    fn new(content: T) -> Self {
        MyBox {
            box_content: content,
        }
    }

    fn copy(&self) -> MyBox<T>
    where
        T: Clone,
    {
        MyBox {
            box_content: self.box_content.clone(),
        }
    }
}

pub fn execute_generic_struct_where() {
    let s = MyBox::new("Hello");
    let n = MyBox::new(42);

    println!("{:?} {:?}", s, n);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
