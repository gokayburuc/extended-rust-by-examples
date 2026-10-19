struct A;

// S() ifadesi bir wrapper ifadesidir
struct SWrapper(A);

//SGen<T>(T) herhangi bir tip ile somutlaştırılabilen generic wrapper ifadesidir
struct SGenericWrapper<T>(T);

// `regular_function`, `_s: S` alır. Fonksiyon adından önce `<T>` YOK
// → bu fonksiyon jenerik değildir, SADECE `S` tipini kabul eder.
// _s ownership ile alınır; fonksiyon scope'u bitince değer düşürülür.
fn regular_function(_s: SWrapper) {}

// `gen_spec_t` biraz aldatıcıdır: `SGen<A>` jenerik bir tipin SOMUTLAŞMIŞ halidir.
// Fonksiyon adından önce `<A>` yok → fonksiyon kendisi jenerik DEĞİL.
// Yani bu fonksiyon yalnızca `SGen<A>` tipini kabul eder.
fn gen_spec_t(_s: SGenericWrapper<A>) {}

// Aynı durum: `SGen<i32>` sabit, somut bir tiptir.
// `i32` zaten somut bir tip; fonksiyon jenerik değildir.
fn gen_spec_i32(_s: SGenericWrapper<i32>) {}

// `<T>` fonksiyon adından ÖNCE geliyor → fonksiyon T üzerinden jeneriktir.
// Her çağrıda T, kullanılan tipe göre derleyici tarafından belirlenir.
// `SGen<T>` argümanı da T'yle aynı parametreyi paylaşır — T = char ise argüman SGen<char> olmalı.
fn generic_function<T>(_s: SGenericWrapper<T>) {}

pub fn execute_generic_function() {
    regular_function(SWrapper(A)); // S ifadesi zaten Sadece a alır

    gen_spec_t(SGenericWrapper(A)); // SGen(A) dan çıkarım SGen<A> - fonksiyonun istediği tip

    gen_spec_i32(SGenericWrapper(6)); // SGen(6) dan çıkarım SGen<i32>

    // ---- Jenerik fonksiyon: T burada belirleniyor ----
    // INFO: TURBOFISH sözdizimi: T'yi açıkça `char` olarak belirtiyoruz.
    // `::<>` görünümü balık yüzgecine benzediği için bu ad verilmiştir.
    generic_function::<char>(SGenericWrapper('a'));

    // Burada da turbofish kullanılabilirdi ama gerek yok:
    // derleyici `SGen('c')` argümanından T = char çıkarımını yapar.
    generic_function(SGenericWrapper('a'));
}
