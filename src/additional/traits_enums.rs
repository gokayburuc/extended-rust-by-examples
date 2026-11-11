// ============================================================
// KONU     : Enum ile Trait Kullanımı
// KAYNAK   : https://doc.rust-lang.org/rust-by-example/trait.html
// AÇIKLAMA : Enum varyantları üzerine trait implementasyonu yapılarak
//            farklı varyantlar için ortak davranışların tanımlanmasını
//            gösterir.
// ============================================================

use crate::additional::traits_explain::Audio;
use crate::additional::traits_explain::Cat;
use crate::additional::traits_explain::Dog;
enum Animal {
    Cat(Cat),
    Dog(Dog),
}

// TODO:  Box<dyn Trait> snytax
pub fn execute_trait_enum() {
    let animals: Vec<Box<dyn Audio>> = vec![
        Box::new(Cat {
            animal_name: String::from("Tekir"),
        }),
        Box::new(Dog {
            animal_name: String::from("Karabas"),
        }),
    ];

    for h in &animals {
        println!("{}", h.make_voice());
    }
}
