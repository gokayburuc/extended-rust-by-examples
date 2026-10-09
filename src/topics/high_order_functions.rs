fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn execute_high_order_function() {
    println!("Find the sum of all the numbers with odd squares under 1000");

    let upper = 1000;

    let mut acc = 0;

    // TODO: check 0..
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n;
        }
    }

    println!("imperative style: {}", acc);

    let sum: u32 = (0..)
        .take_while(|&n| n * n < upper)
        .filter(|&n| is_odd(n * n))
        .sum();
    println!("Functional style: {}", sum);
}
