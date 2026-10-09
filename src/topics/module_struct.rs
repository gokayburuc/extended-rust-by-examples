mod my_awesome {
    // Genel (public) alanlara sahip bir yapı; dışarıdan erişilebilir
    pub struct OpenBox<T> {
        // İçerik alanı herkese açık, modül dışından da erişilebilir
        pub contents: T,
    }

    // Özel (private) alanlara sahip bir yapı; alanlar sadece modül içinden erişilebilir
    pub struct ClosedBox<T> {
        // İçerik alanı gizli; dışarıdan doğrudan erişilemez
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // Yeni bir ClosedBox oluşturan kurucu (constructor) fonksiyon
        pub fn new(contents: T) -> ClosedBox<T> {
            // Özel alana modül içinden erişim sağlanarak yapı döndürülür
            ClosedBox { contents: contents }
        }
    }
}

pub fn execute_module_struct() {
    // OpenBox yapısı doğrudan alan ismiyle oluşturulabilir (contents public olduğu için)
    let open_box = my_awesome::OpenBox {
        contents: "public information",
    };

    // Public alana modül dışından erişim mümkündür
    println!("The open box contains: {}", open_box.contents);

    // ClosedBox'ın alanı private olduğu için yalnızca new fonksiyonuyla oluşturulabilir
    let _closed_box = my_awesome::ClosedBox::new("classified information");
    // DİKKAT: contents alanı private olduğu için bu erişim derleme hatası verir
    // println!("The closed box contains: {}", _closed_box.contents);
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod cool {
    pub use crate::topics::module_struct::deeply::nested::function;
}
pub fn execute_seond_module_struct() {
    cool::function();
}
