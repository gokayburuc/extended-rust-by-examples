/// `FnOnce` trait'i, bir kere çağrılabilen (veya hiç çağrılmayan) fonksiyonlar/closures için kullanılır.
/// Bu fonksiyon, `F` tipindeki bir closure'ı alır ve bir kere çağırır.
pub fn execute_closures_apply<F>(f: F)
where
    F: FnOnce(), // `FnOnce`: Closure sadece bir kere çağrılabilir (örneğin, move semantics ile ownership alınmış değişkenler varsa).
{
    f(); // Closure'ı çağır.
}

// Bu fonksiyon, `i32` alıp `i32` döndüren bir closure'ı 3 ile çağırır ve sonucu döndürür.
pub fn execute_apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32, // `Fn`: Closure birden fazla kere çağrılabilir ve immutable referanslar kullanır.
{
    f(3) // Closure'a 3 değerini geçir ve sonucu döndür.
}

/// Bu fonksiyon, closure'ların ownership, mutable state ve `FnOnce`/`Fn`/`FnMut` trait'lerini gösterir.
pub fn execute_closure_as_input() {
    use std::mem; // `mem::drop`, bellekteki bir değeri erken serbest bırakmak için kullanılır.

    let greeting = "hello"; // `greeting`: String slice (`&str`), immutable ve stack'te saklanır.

    // `to_owned()`: `&str`'yi `String`'e dönüştürür (heap'te saklanır).
    // `farewell`: Mutable `String`, ownership bu scope'a aittir.
    let mut farewell = "goodbye".to_owned();

    // `diary`: Bir closure tanımı. `FnOnce` trait'ini uygular çünkü `farewell`'u move eder (ownership alır).
    let diary = || {
        println!("I said {}", greeting); // `greeting`'e immutable referansla erişilir.

        // `farewell`'a mutable referansla erişilir ve değiştirilir.
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);

        println!("Now i can sleep, zzzz");

        // `mem::drop(farewell)`: `farewell`'u bellekten erken serbest bırakır.
        // Bu, `farewell`'un ownership'ini elden çıkarır ve `diary` closure'ının `FnOnce` olmasını sağlar.
        mem::drop(farewell);
    };

    // `diary` closure'ını `execute_closures_apply` fonksiyonuna geçir ve çağır.
    execute_closures_apply(diary);

    // `double`: Basit bir closure, `x` değerini 2 ile çarpar.
    // `Fn` trait'ini uygular çünkü immutable referanslar kullanır ve birden fazla kere çağrılabilir.
    let double = |x| 2 * x;

    // `execute_apply_to_3` fonksiyonunu `double` closure'ı ile çağır.
    println!("3 doubled: {}", execute_apply_to_3(double));
}

// TODO: traits
// TODO: Closures
// TODO: std::mem
// TODO: ownership
