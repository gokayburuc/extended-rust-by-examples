struct Miles(f64);
struct Kilometers(f64);

impl Miles {
    pub fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 * 1.609344)
    }
}

impl Kilometers {
    pub fn to_miles(&self) -> Miles {
        Miles(self.0 / 1.609344)
    }
}

fn is_a_marathon(distance: &Miles) -> bool {
    distance.0 >= 26.2
}

fn execute_new_type_idiom() {
    let distance = Miles(30.0);
    let distance_km = distance.to_kilometers();

    println!("Is a marathon? {}", is_a_marathon(&distance));
    println!("Is a marathon? {}", is_a_marathon(&distance_km.to_miles()));
    //  TODO: method implementation of the code below
    // println!("Is a marathon? {}", is_a_marathon(&distance_km));
}

pub fn execute_new_type_idiom_second() {
    // TODO: explain distance_as_primitive_1 and distance_as_primitive_2 difference
    let distance = Miles(42.0);
    let distance_as_primitive_1: f64 = distance.0;
    let Miles(distance_as_primitive_2) = distance;
}
