use crate::smallproject::platecity::match_city;

mod additional;
mod smallproject;
mod topics;

fn main() {
    // additional::hashmaps::execute_hashmap();

    // short call
    // hashmaps::execute_hashmap();

    smallproject::platecity::match_city(22);
    match_city(39);
    match_city(41);

    additional::hashmaps::execute_hashmap();
}
