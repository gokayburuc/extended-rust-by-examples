use deeply::nested::nested_function as other_function;

mod deeply {
    pub mod nested {
        pub fn nested_function() {
            println!("called deeply::nested::function()");
        }
    }
}

fn function() {
    println!("called function()");
}

fn execute_use_declaration() {
    other_function();
    println!("Entering block");

    {
        use crate::topics::use_declaration::deeply::nested::nested_function;
        nested_function();

        println!("Leaving block");
    }

    function();
}
