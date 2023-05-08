struct Point {
    x: i32,
    y: i32,
}

fn mutiples_functions(){
    let x = 56;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let y = 1;

    match y {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    // Desctructuring
    let origin = Point { x: 50, y: 150 };

    match origin {
        Point { x, y } => println!("({},{})", x, y),
    };

    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    };

    // Ignore variables links
    let some_value: Result<i32, &'static str> = Err("There was an error");
    match some_value {
        Ok(value) => println!("got a value: {}", value),
        Err(_) => println!("an error occured"),
    }    

    // ref and ref mut
    let x = 5;

    match x {
        ref r => println!("Got a reference to {}", r),
    }

    let mut x = 5;

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    let x = 4;
    match x {
        1 ..= 5 => println!("one through five"),
        _ => println!("something else"),
    }
}