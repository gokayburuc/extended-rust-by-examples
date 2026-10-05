pub fn execute_iterator_any() {
    let first_vector = vec![1, 2, 3];
    let second_vector = vec![1, 2, 3];

    // WARN:  iter().any() ifadesi &i32 getirdiği için bu ifadeyi daha sonra i32'ye
    println!(
        "2 in first_vector: {}",
        first_vector.iter().any(|&x| x == 2)
    );

    // WARN:  into_iter().any() ifadesi doğrudan i32 getirir desctrure gerektirmez
    println!(
        "2 in second_vector: {}",
        second_vector.into_iter().any(|x| x == 2)
    );

    println!("First vector len: {}", first_vector.len());

    println!("First element of first_vector is: {}", first_vector[0]);

    let first_array = [1, 2, 3];
    let second_array = [4, 5, 6];

    println!("2 in first_array: {}", first_array.iter().any(|&x| x == 2));
    println!(
        "2 in second_array: {}",
        second_array.into_iter().any(|x| x == 2)
    );
}
