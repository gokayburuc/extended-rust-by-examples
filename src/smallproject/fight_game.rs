use rand::Rng;

// Her silahın bir saldırı yapma davranışı var. Bu da generic yapsının dayandığı contract
trait Attack {
    // silahın taban gücü: her silah tipine göre mevcut bir değer vardır.
    fn base_power(&self) -> i32;

    // saldırı açıklaması (loglar için kullanılacak)
    fn attack_name(&self) -> &str;
}

struct Sword;

impl Attack for Sword {
    fn base_power(&self) -> i32 {
        12
    }

    fn attack_name(&self) -> &str {
        "Sword Attack"
    }
}

struct Bow;

impl Attack for Bow {
    fn base_power(&self) -> i32 {
        9
    }
    fn attack_name(&self) -> &str {
        "Bow Attack"
    }
}

struct MagicStaff;
impl Attack for MagicStaff {
    fn base_power(&self) -> i32 {
        15
    }

    fn attack_name(&self) -> &str {
        "Magic Staff Attack"
    }
}

struct Fighter<W> {
    character_name: String,
    character_energy: i32,
    character_weapon: W,
}

impl<W: Attack> Fighter<W> {
    fn new(fighter_name: &str, fighter_energy: i32, fighter_weapon: W) -> Self {
        Fighter {
            character_name: fighter_name.to_string(),
            character_energy: fighter_energy,
            character_weapon: fighter_weapon,
        }
    }

    fn roll_damage(&self) -> i32 {
        // FIX: rand::thread_rng() - deprecated
        let mut rng = rand::rng();

        // FIX: rand::thread_rng::gen_range - deprecated
        rng.random_range(0..=self.character_weapon.base_power())
    }

    fn take_damage(&mut self, dmg: i32) -> bool {
        self.character_energy -= dmg;
        self.character_energy > 0
    }

    fn is_alive(&self) -> bool {
        self.character_energy > 0
    }
}

impl Fighter<MagicStaff> {
    fn reckless_blast(&mut self) -> i32 {
        let dmg = self.roll_damage();
        self.character_energy -= 3;
        dmg
    }
}

fn encounter<A: Attack, B: Attack>(a: &mut Fighter<A>, b: &mut Fighter<B>, round: u32) {
    println!("--- Tur: {} ---", round);

    let dmg_a = a.roll_damage();
    let dmg_b = b.roll_damage();

    println!(
        "{} ({}): {} hasar üretti",
        a.character_name,
        a.character_weapon.attack_name(),
        dmg_a
    );

    println!(
        "{} ({}): {} hasar üretti",
        b.character_name,
        b.character_weapon.attack_name(),
        dmg_b
    );

    let b_alive = b.take_damage(dmg_a);
    let a_alive = a.take_damage(dmg_b);

    println!(
        "Sonuç: {} -> {} enerji | {} -> {} enerji",
        a.character_name, a.character_energy, b.character_name, b.character_energy
    );

    if !a_alive {
        println!("{} Elendi!", a.character_name);
    }
    if !b_alive {
        println!("{} Elendi!", b.character_name);
    }
}

pub fn start_game() {
    let mut warrior = Fighter::new("Olga", 100, Sword);
    let mut mage = Fighter::new("Tatiana", 85, MagicStaff);
    // let mut archer = Fighter::new("Nadya", 92, Bow);

    println!(
        "Dövüş başlıyor: {} vs {}!",
        warrior.character_name, mage.character_name
    );

    let mut round = 1;

    while warrior.is_alive() && mage.is_alive() {
        encounter(&mut warrior, &mut mage, round);

        if round >= 20 {
            println!("Savaş uzadı. Bu yüzden beraberlik verildi.");
            break;
        }

        round += 1;
    }

    println!("GAME OVER!");

    if warrior.is_alive() && !mage.is_alive() {
        println!("Kazanan {}!", warrior.character_name);
    } else if mage.is_alive() && !warrior.is_alive() {
        println!("Kazanan {}!", mage.character_name);
    } else {
        println!("Savaş Berabere bitti!");
    }
}
