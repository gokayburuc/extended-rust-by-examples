mod my_mod {
    // Özel (private) fonksiyon: yalnızca my_mod modülü içinden erişilebilir.
    fn private_function() {
        println!("Callede my_mod::private_function()");
    }

    // Herkese açık (public) fonksiyon: modül dışından da erişilebilir.
    pub fn function() {
        println!("called my_mod::function()");
    }

    // Aynı modül içindeki özel fonksiyona dolaylı erişim sağlar.
    pub fn indirect_access() {
        print!("called my_mod::indirect_access() that\n>");
        private_function();
    }

    // İç içe geçmiş (nested) alt modül.
    pub mod nested {
        // Alt modülün herkese açık fonksiyonu.
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        // Alt modülün özel fonksiyonu: sadece nested içinden erişilebilir.
        #[allow(dead_code)]
        fn private_function() {
            println!("called my_mod::nested::private_function()");
        }

        // Yalnızca my_mod modülü (ve alt modülleri) içinde erişilebilir.
        pub(in crate::topics::modules::my_mod) fn public_function_in_my_mod() {
            print!("called my_mod::nested::public_function_in_my_mod()")
        }

        // Yalnızca kendi modülünün (nested) içinde erişilebilir — private ile eşdeğer.
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }
        // Yalnızca üst modülü olan my_mod içinde erişilebilir.
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }

    // my_mod içinden nested modülünün kısıtlı erişimli fonksiyonlarını çağırır.
    pub fn call_public_function_in_my_mod() {
        print!("called my_mod::call_public_function_in_my_mod() that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // Tüm crate (proje) kapsamında erişilebilir, ama dış crate'lerden erişilemez.
    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate()");
    }

    // Özel alt modül: dışarıdan erişilemez, bu yüzden içindeki fonksiyonlar dışarıya görünmez.
    mod private_nested {
        // Teknik olarak public olsa bile, modül gizli olduğu için crate dışından erişilemez.
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }

        // Crate kapsamında tanımlı olsa bile, gizli modül içinde olduğu için yalnızca my_mod içinde kullanılabilir.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

// Crate kökünde (dosya kapsamında) tanımlı bağımsız bir fonksiyon.
fn function() {
    println!("called function()");
}

// Modülleri tanıtan ve çağıran gösteri (demo) fonksiyonu.
pub fn execute_modules() {
    function(); // Crate kökündeki function() çağrılır.

    // my_mod içinden nested'ın kısıtlı fonksiyonlarını çağırır.
    my_mod::call_public_function_in_my_mod();
    my_mod::function(); // my_mod'ün herkese açık fonksiyonu.
    my_mod::indirect_access(); // Özel fonksiyona modül içinden dolaylı erişim.
    my_mod::nested::function(); // Alt modülün herkese açık fonksiyonu.
    my_mod::public_function_in_crate(); // Crate kapsamındaki fonksiyon.
}
