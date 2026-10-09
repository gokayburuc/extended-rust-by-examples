// Kök seviyede tanımlanmış bir fonksiyon; crate'in en üst modülüne aittir
fn function() {
    println!("called function()");
}

// 'cool' adında bir modül
mod cool {
    // 'cool' modülünün public fonksiyonu; dışarıdan erişilebilir
    pub fn function() {
        println!("called cool::function()");
    }
}

// 'my_mod' adında bir modül
mod my_mod {
    // 'my_mod' içindeki public fonksiyon
    pub fn function() {
        println!("called my_mod::function()");
    }

    // 'my_mod'ün içinde iç içe geçmiş 'cool' modülü
    pub mod cool {
        // İç modüldeki public fonksiyon
        pub fn cool_function() {
            println!("called my_mod::cool::function()");
        }
    }

    // Dolaylı çağrıları gösteren fonksiyon
    pub fn indirect_call() {
        println!("called my_mod::indirect_call()\n");

        // 'self' ile mevcut modüldeki (my_mod) fonksiyona açıkça başvurulur
        self::function();

        // 'self::' öneki olmadan da aynı anlama gelir; mevcut modül içinde arama yapılır
        function();

        // Mevcut modülün içindeki 'cool' alt modülündeki fonksiyona erişilir
        self::cool::cool_function();

        // 'super' ile bir üst modüle (burada crate'in kökü) geçilir ve oradaki fonksiyon çağrılır
        super::function();

        {
            // 'use' ile crate'in kökündeki 'cool' modülündeki fonksiyon
            // 'root_function' adıyla bu kapsama (scope) içeri aktarılır
            use crate::topics::modules_super_self::cool::function as root_function;

            // Yeni adıyla kökteki fonksiyon çağrılır
            root_function();
        }
    }
}

// Bu dosyanın dışından çağrılacak genel yürütme fonksiyonu
pub fn execute_module_super_self() {
    // 'my_mod' içindeki dolaylı çağrı fonksiyonunu tetikler
    my_mod::indirect_call();
}
