use std::collections::HashMap;

pub fn execute_hashmap() {
    // Define a new empty HashMap
    let mut city_plate = HashMap::new();

    city_plate.insert("Edirne", 22);
    city_plate.insert("Kırklareli", 39);
    city_plate.insert("Çanakkale", 17);
    city_plate.insert("Bursa", 16);

    match city_plate.get("Bursa") {
        Some(v) => println!("Plate Number: {}", v),
        None => println!("Plaka bulunamadı!"),
    }

    // idiomatic way to get value
    if let Some(v) = city_plate.get("Bursa") {
        println!("Bursa: {}", v);
    }

    println!("Eleman Sayısı : {}", city_plate.len());

    city_plate.remove("Çanakkale");
    println!("Plate info: {:?}", city_plate);
}
