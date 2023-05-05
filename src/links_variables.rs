fn links_variables(){
    let x = 5;
    println!("The value of x is: {}", x);

    let (x, y) = (1, 2);
    println!("The value of x and y are: {}, {}", x, y);

    // Typing variable
    let x:i32 = 5;
    println!("The value of x type is: {}", x);

    // Untyping variable
    let x = 10;
    println!("The value of x type is: {}", x);


    // Mutable variables 
    let mut z = 5;
    println!("The value of z is: {}", z);
    z = 10;
    
    println!("The value of z mutable is: {}", z);
}