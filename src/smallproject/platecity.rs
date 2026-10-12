use std::collections::HashMap;

pub fn match_city(plate_number: i32) {
    let mut city_info: HashMap<&str, i32> = HashMap::new();

    city_info.insert("Edirne", 22);
    city_info.insert("Kırklareli", 39);
    city_info.insert("Bursa", 16);
    city_info.insert("Çanakkale", 17);
    city_info.insert("Tekirdağ", 59);
    city_info.insert("Balıkesir", 10);
    city_info.insert("İstanbul", 34);
    city_info.insert("Kocaeli", 41);
    city_info.insert("Sakarya", 54);
    city_info.insert("Yalova", 77);
    city_info.insert("Kütahya", 43);

    // reverse the Hashmap for k,v -> v,k
    let reversed_city_info: HashMap<i32, &str> =
        city_info.iter().map(|(&il, &plaka)| (plaka, il)).collect();

    match reversed_city_info.get(&plate_number) {
        Some(&il) => println!("City {} Plate Number: {} ", il, plate_number),
        None => println!("Geçersiz Plaka: {}", plate_number),
    };
}

