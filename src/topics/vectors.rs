#[allow(dead_code)]
pub fn execute_vector() {
    let mut cities = vec!["Edirne", "Kırklareli", "Tekirdağ", "Çanakkale"];
    println!("City List: {:?}", &cities);

    for city in &cities {
        println!("City Name : {}", &city);
    }

    cities.push("İstanbul");
    println!("Istanbul added : {:?}", &cities);

    cities.remove(3);
    println!("Index 3 removed : {:?}", &cities);

    let second_cities = vec!["Konya", "Kayseri", "Aksaray"];

    let third_cities = [cities, second_cities].concat();

    println!("{:?}", third_cities);
}
