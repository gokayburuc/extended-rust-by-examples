use std::mem;
pub fn execute_capturing_closures() {
    // String türünde bir değişken tanımlıyoruz: "green".
    let color = String::from("green");

    // `color` değişkenini yakalayan bir closure (kapanış).
    // Closure, `color`ı ödünç alarak (immutable borrow) kullanır.
    let color_print = || println!("color: {}", color);
    color_print(); // Çıktı: "color: green"

    // `color`a yeni bir immutable referans (ödünç) alıyoruz.
    // Bu sorunsuz çalışır, çünkü `color_print` closure'ı `color`ı sadece okur.
    let _reborrow = &color;
    color_print(); // Çıktı: "color: green"

    // `color` değişkenini taşınıyoruz (move ediyoruz).
    // Artık `color` kullanılamaz, çünkü `color_print` closure'ı `color`ı yakalamıştı.
    // Ancak, `color_print` artık kullanılmayacağı için derleyici buna izin verir.
    let _color_moved = color;

    // --- Sayaç (Counter) Örneği ---
    // Değiştirilebilir (mutable) bir sayaç değişkeni.
    let mut count = 0;

    // `count`ı yakalayan ve değiştiren bir closure.
    // Closure, `count`ı mutable olarak ödünç alır.
    let mut increment_counter = || {
        count += 1; // `count`ı değiştirir.
        println!("count: {}", count);
    };

    increment_counter(); // Çıktı: "count: 1"

    // TODO: Neden hata verir?
    // UYARI: Bu satır hata verir!
    // let _reborrow = &count;
    // increment_counter();
    //
    // AÇIKLAMA:
    // `increment_counter` closure'ı, `count`ı mutable olarak ödünç aldığından,
    // `count`a başka bir referans (`_reborrow`) almak, Rust'ın ödünç alma kurallarını ihlal eder.
    // Rust, aynı anda sadece bir tane mutable referans izin verir.
    // Bu yüzden, `_reborrow` satırı yorum satırına alındı.

    // `count`a yeni bir mutable referans alıyoruz.
    // Bu çalışır, çünkü `increment_counter` artık kullanılmayacak.
    let _count_reborrowed = &mut count;

    // --- Box<T> Örneği ---
    // `Box` içinde bir değer tanımlıyoruz: 3.
    let movable = Box::new(3);

    // `movable`ı yakalayan ve tüketen (consume) bir closure.
    // Closure, `movable`ı sahiplenir (move eder) ve `mem::drop` ile bellekten siler.
    let consume = || {
        println!("movable: {:?}", movable); // Çıktı: "movable: 3"
        mem::drop(movable); // `movable` bellekten silinir.
    };

    consume(); // Closure çağrılır ve `movable` tüketilir.

    // `movable` artık kullanılamaz, çünkü `consume` tarafından tüketildi.
}

pub fn execute_capturing_move() {
    // `Vec` kopyalanamaz (non-copy semantics).
    let haystack = vec![1, 2, 3];

    // `move` anahtar kelimesi, `haystack` değişkenini closure'a taşır (sahiplenir).
    let contains = move |needle| haystack.contains(needle);

    // `contains` closure'ını çağırarak `haystack` içinde `1` var mı diye kontrol ediyoruz.
    println!("{}", contains(&1));
    // `contains` closure'ını çağırarak `haystack` içinde `4` var mı diye kontrol ediyoruz.
    println!("{}", contains(&4));

    // Aşağıdaki satırın yorumunu kaldırırsanız, derleme hatası oluşur.
    // Çünkü `haystack` değişkeni, `move` nedeniyle closure'a taşındı ve artık kullanılamaz.
    // println!("There're {} elements in vec", haystack.len());

    // Eğer `move` anahtar kelimesini closure'dan kaldırırsanız,
    // closure `haystack` değişkenini ödünç alır (immutable borrow) ve `haystack` hala kullanılabilir hale gelir.
    // Bu durumda yukarıdaki satırın yorumunu kaldırsanız bile hata oluşmaz.
}
