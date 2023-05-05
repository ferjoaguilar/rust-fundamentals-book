// Function to receive an argument
fn print_number(x: i32) {
    println!("x is: {}", x);
}

// Function to add two numbers
fn add_numbers(x: i32, y: i32){
    println!("The add to x and are {}", x + y);
}

// Function to retunr a value
fn add_one(x:i32) -> i32{
    x + 1
}

fn add_two(x:i32) -> i32{
    return x + 2;
}

// Divergent functions
fn diverges() -> ! {
    panic!("This function never returns!");
}


fn run_all_functions() {
    print_number(20);
    add_numbers(20, 50);

    let x:i32 = add_one(5);
    println!("The value of x is: {}", x);

    let y:i32 = add_two(5);
    println!("The value of y is: {}", y);

    diverges();
}
