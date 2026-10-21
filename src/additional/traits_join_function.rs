pub trait Audio {
    fn make_voice(&self) -> String;
}

pub struct CatAgain {
    pub cat_name: String,
}

pub trait PrintName {
    fn name_print(&self) -> String;
}

impl Audio for CatAgain {
    fn make_voice(&self) -> String {
        format!("{} : Meow!", self.cat_name)
    }
}

impl PrintName for CatAgain {
    fn name_print(&self) -> String {
        format!("{}", self.cat_name)
    }
}

pub fn report<T: Audio + PrintName>(our_cat: &T) {
    println!(
        " Name: {} - {} ",
        our_cat.name_print(),
        our_cat.make_voice()
    );
}
