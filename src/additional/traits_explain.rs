pub trait Audio {
    fn make_voice(&self) -> String;
}

struct Cat {
    animal_name: String,
}
struct Dog {
    animal_name: String,
}

impl Audio for Cat {
    fn make_voice(&self) -> String {
        format!("{}: Meow!", self.animal_name)
    }
}

impl Audio for Dog {
    fn make_voice(&self) -> String {
        format!("{}: Hav!", self.animal_name)
    }
}

// Trait bound -  impl Trait
fn speak_trait_bound(animal: &impl Audio) {
    println!("{}", animal.make_voice())
}

// Generic + trait bound
fn speak_generic<T: Audio>(animal: &T) {
    println!("{}", animal.make_voice());
}

pub fn execute_trait_explanation() {
    let sample_cat = Cat {
        animal_name: String::from("Snezhka"),
    };
    let sample_dog = Dog {
        animal_name: String::from("Barsik"),
    };

    println!("{}", sample_cat.make_voice());
    println!("{}", sample_dog.make_voice());

    // trait bound
    speak_trait_bound(&sample_cat);
    speak_generic(&sample_dog);
}
