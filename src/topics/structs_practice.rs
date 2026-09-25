pub fn execute_struct_practice() {
    // NOTE: inside function struct
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    #[allow(unreachable_patterns, unused)]
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { x: i, y } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we dont care about x", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    let Foo { x: x0, y: y0 } = faa;

    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;

    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
