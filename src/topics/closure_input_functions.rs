// Bu fonksiyon "public": dış modüllerden de çağrılabilir.
pub fn execute_closure_input_function() {
    // Bir closure tanımlıyoruz: argümansız, değer döndürmüyor.
    // Derleyici bunun için ANONİM bir tip üretir ve Fn trait'ini implement eder.
    let closure = || println!("I'm a closure!");

    // Aynı fonksiyona önce closure, sonra sıradan bir fn geçiriyoruz.
    // İkisi de Fn() trait'ini sağladığı için kabul edilir.
    call_this(closure); // closure anonim tiptir
    call_this(sample_function); // fn pointer ise fn item tipindedir

    // Not: `closure` FnOnce değildir/taşınmaz, sadece çağrılır;
    // isteseydik tekrar da çağırabilirdik (Fn birden çok kez çağrılabilir).
}

// Sıradan (closure olmayan) bir fonksiyon.
fn sample_function() {
    println!("I'm a function!");
}

// "call_this" herhangi bir ÇAĞRILABİLİR şeyi kabul eder,
// ama şartı var: argümansız çağrılabilen ve bir şey döndürmeyen (Fn()).
// F generic olduğu için closure'ın anonim tipini de kabul edebilir.
fn call_this<F: Fn()>(f: F) {
    // WARN: call_this fonksiyonu sadece function ve closure yapılarını döndürür.
    // Variable döndürmez ve hata verir.
    f(); // gelen şeyi çağırıyoruz.
}
