// ============================================================
// KONU     : Trait Varsayılan Metotlar
// KAYNAK   : https://doc.rust-lang.org/rust-by-example/trait.html
// AÇIKLAMA : Trait tanımı içinde varsayılan (default) metot gövdesi
//            eklenerek implementasyonu isteğe bağlı hale getirmeyi gösterir.
// ============================================================

trait Greeting {
    fn greet(&self) {
        println!("Hello!");
    }
}

struct Bulgarian;
struct Russian;
struct Turkish;
struct Greek;
struct English;

// INFO: if you give no value uses the default value - hello
impl Greeting for English {}

impl Greeting for Greek {
    fn greet(&self) {
        println!("Kalimera!");
    }
}

impl Greeting for Bulgarian {
    fn greet(&self) {
        println!("Zdravej!");
    }
}

pub fn execute_default_method_trait() {
    English.greet();
    Greek.greet();
    Bulgarian.greet();
}
