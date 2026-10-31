// FIX: trait Display

// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }

// std::fmt içindeki Debug trait'ini projeye dahil ediyoruz
use std::fmt::Debug;

// Rectangle yapısı (struct) tanımlanıyor ve #[derive(Debug)] ile otomatik Debug implementasyonu sağlanıyor
#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

// Kullanılmayan kod uyarılarını engellemek için #[allow(dead_code)] eklenmiş Triangle yapısı
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// Alan hesaplama yeteneği kazandıran HasArea trait'i
trait HasArea {
    fn area(&self) -> f64;
}

// Rectangle için HasArea trait'inin implementasyonu
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

// Debug trait'ini uygulayan herhangi bir türü ekrana yazdıran genel (generic) fonksiyon
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t)
}

// HasArea trait'ini uygulayan herhangi bir türün alanını döndüren genel fonksiyon
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

// Ana çalıştırma fonksiyonu
pub fn execute_generic_bounds() {
    // Örnek bir Rectangle nesnesi oluşturuluyor
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    // Örnek bir Triangle nesnesi oluşturuluyor
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    // Bu kısım çalışacaktır çünkü Rectangle Debug trait'ini türetmektedir (derive)
    print_debug(&rectangle);

    // Rectangle nesnesinin alanı hesaplanıp ekrana yazdırılıyor
    println!("Area: {}", area(&rectangle));
}
