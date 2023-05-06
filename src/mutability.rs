fn mutability(){
    // copy content to main.rs
    // Mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // Mutable variable from pointer
    let mut x = 5;
    let y = &mut x;
    println!("The value of y is: {}", y);

    let x = Arc::new(5);
    println!("The using arc value of x is: {}", x);
    let y = x.clone();
    println!("The using arc value of y is: {}", y);
}