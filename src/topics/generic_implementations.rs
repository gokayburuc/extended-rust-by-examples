// `S` isimli somut bir unit struct.
struct S;

// `T` tipinde tek alan taşıyabilen jenerik bir tuple struct.
struct GenericVal<T>(T);

// `GenericVal<f32>` için impl: sadece T = f32 olan GenericVal'ları hedefler.
// Buradaki `<f32>` tipin ADINDA gelir, impl'in değil → jeneriklik yoktur.
impl GenericVal<f32> {} // `f32` açıkça belirtilmiş.

// Aynı mantık: bu impl yalnızca `GenericVal<S>` tipi içindir.
// S, yukarıda tanımladığımız somut tip.
impl GenericVal<S> {}

// İçine yazılan metotlar, GenericVal'in T'ye bağlı TÜM varyantlarında çalışır.
impl<T> GenericVal<T> {}

// Somut bir struct: 'val' alanı her f64'tür
struct Val {
    val: f64,
}

// Jenerik bir struct: `gen_val` alanının tipi henüz belirsiz (T).
struct GenVal<T> {
    gen_val: T,
}

// `Val` tipine metodlar ekliyoruz (impl bloğu, tipin kendisiyle ilişkili).
impl Val {
    // `&self`: metot, struct'ı MOVE etmez; ödünç alır (immutable borrow).
    // Dönüş tipi `&f64`: değer sahipliği değil, bir referans döndürürüz.
    fn value(&self) -> &f64 {
        // `&self.val`: `val` alanının referansını geri veriyoruz.
        // `self.val` yazsaydık ownership'i taşımaya çalışırdık ve hata alırdık.
        &self.val
    }
}

// `impl<T>`: T, bu impl için tanımlanan tip parametresi.
// Bu bloktaki metotlar, GenVal'in HER varyantında (GenVal<i32>, GenVal<f64>, ...)
// otomatik olarak çalışır.
impl<T> GenVal<T> {
    // Dönüş tipi `&T`: hangi tipe somutlaşırsa, o tipin referansını döndürür.
    fn value(&self) -> &T {
        // `&self.gen_val`: alanın tipi T'dir; referans döndürmekownership'i bozmaz.
        &self.gen_val
    }
}

pub fn execute_generic_implementation() {
    // `Val` somut tipte bir değer oluştur (struct literal sözdizimi).
    let x = Val { val: 3.0 };

    // `GenVal<i32>` oluştur: T, `3i32` literalından örtük olarak çıkarılır.
    // (`3i32` soneki sayının tipini açıkça i32 yapar.)
    let y = GenVal { gen_val: 3i32 };

    // Her iki value() çağrısı da yalnızca ÖDÜNÇ alır; x ve y hâlâ sahibiz.
    // Referanslar println! içine geçer; çağrı sonrası x ve y kullanılabilir kalır.
    println!("{}, {}", x.value(), y.value());
}
