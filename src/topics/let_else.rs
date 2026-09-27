use std::str::FromStr;

pub fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };

    (count, item)
}

// TODO: check the second function with Le Chat

pub fn execute_get_count_item() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}
