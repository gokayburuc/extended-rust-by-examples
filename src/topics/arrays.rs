#[allow(dead_code)]

pub fn execute_array() {
    // array definition
    let mut my_array: [i32; 3] = [33, 44, 55];

    // print array
    println!("My Array: {:?}", my_array);

    // print first item
    println!("First {}", my_array[0]);

    // assign new value to item index 0
    my_array[0] = 32;

    println!("My Array after change: {:?}", my_array);

    // make array mutable
    let second_array: [&mut i32; 3] = my_array.each_mut();

    *second_array[2] = 333;
    println!("Second Array: {:?}", second_array);
}
