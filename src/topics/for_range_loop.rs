pub fn execute_for_range_loop() {
    for n in 1..101 {
        if n % 15 == 0 {
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn execute_reverse_range_loop() {
    // NOTE: reverse iterations direction 10,9,8,...
    for i in (1..10).rev() {
        // WARN: 10 not included
        println!(" {} fizzbuzz", i);
    }
}

pub fn execute_for_iter() {
    let names = vec!["Hans", "Tony", "Frank", "Herkel"];

    // iterates the items in names Vector
    for name in names.iter() {
        match name {
            &"Tony" => println!("{} - There is a rustacean among us!", name),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

pub fn execute_into_iter() {
    let names = vec!["Hans", "Tony", "Frank", "Herkel"];

    for name in names.into_iter() {
        match name {
            "Frank" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

pub fn execute_into_mut() {
    let mut names = vec!["Hans", "Tony", "Frank", "Herkel"];

    // NOTE: changes item at place with new value
    for name in names.iter_mut() {
        *name = match name {
            &mut "Hans" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
