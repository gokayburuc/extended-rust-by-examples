#[allow(dead_code)]
pub fn execute_iflet() {
    // TODO: Some(T) -> Burada `Some(T)` türünde bir `Option` örneği oluşturulmalı.
    let number = Some(7); // `number` değişkenine `Some(7)` atanmış, yani bir değer içeriyor.

    // TODO: Option<> -> Burada `Option` türünde değişkenler tanımlanmalı.
    let letter: Option<i32> = None; // `letter` değişkeni `None` olarak başlatıldı, yani hiçbir değer içermiyor.
    let emoticon: Option<i32> = None; // `emoticon` değişkeni de `None` olarak başlatıldı.

    // `number` değişkeni `Some(i)` ile eşleşirse, `i` değeri yazdırılır.
    if let Some(i) = number {
        println!("Matched {:?}!", i); // Çıktı: "Matched 7!"
    }

    // `letter` değişkeni `Some(i)` ile eşleşmezse, else bloğu çalışır.
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter"); // Çıktı: "Didn't match a number. Let's go with a letter"
    }

    let i_like_letters = false; // `i_like_letters` değişkeni `false` olarak ayarlandı.

    // `emoticon` değişkeni `Some(i)` ile eşleşmezse, `i_like_letters` kontrol edilir.
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!"); // Çıktı: "I don't like letters. Let's go with an emoticon :)!"
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

pub fn execute_if_let_second() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // if with let Foo::Bar checks
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // WARN: this will print no value
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }
}
