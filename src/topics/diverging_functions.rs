fn diverging_func() -> ! {
    panic!("This call never returns.");
}

fn some_func() {
    ()
}

pub fn execute_this() {
    let _a: () = some_func();
    println!("This function returns and you can see this line");
}

#![feature(never_type)]
pub fn execute_also_this() {
    let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
}


pub fn execute_this_last() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true =>i, 
                false => continue,
            };
            acc += addition;
        }
        acc 
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
