// A isimli concrete(somut) bir tip tanımlıyoruz.
// Tamamen boş
struct A;

// Single() ifadesi A ifadesinin etrafında bir wrapper ifadesidir
// Single() ifadesi sadece A isimli tip ile kullanılabilir Generic ifadesi değildir.
struct Single(A);

// <T> burada T'nin ilk kullamınıdan önce geliyor
// SingleGen bir generic tiptir, T ise henüz bilinmeyen bir tip parametresidir. Her değeri alabilir.
// T yerine her değer kullanılabilir. Hatta başka bir concrete tip olan A gibi ifadeler de kullanılabilir.

struct SingleGen<T>(T);

fn execute_generics_sample() {
    // Single ifadesi concrete bir tiptir ve yalnızca A değeri alır
    // Single ifadesinin içerisinde i32 tipinde 5 gibi bir değer yazdığında bu struct yapısı çalışmaz
    let _s = Single(A);

    // tip parametresi açıkça - explicitly belirtilmiş ve SingleGen<char> yapısındadır
    // değişken tipini biz belirtiyoruz.
    let _char: SingleGen<char> = SingleGen('a');

    // tip parametresi örtük - implicit şekilde belirtilmiş.
    // Rust argümandan çıkarım yapar ve SingleGen<A> olarak belirler.
    let _t = SingleGen(A);
    let _number = SingleGen(6);
    let _character = SingleGen('a');
    let _option = SingleGen(true);
}
