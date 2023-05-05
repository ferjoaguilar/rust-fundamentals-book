fn type_variables(){
     // Boolean type
     let x = true;
     let y: bool = false;
 
     println!("x: {}", x);
     println!("y: {}", y);
 
     // Char type
     let x = 'x';
     let two_hearts = '💕';
 
     println!("x: {}", x);
     println!("two_hearts: {}", two_hearts);
 
     // Numeric types
     let x = 42; // x has type i32
     let y = 1.0; // y has type f64
 
     println!("x: {}", x);
     println!("y: {}", y);
 
     // Arrays
     let a = [1, 2, 3]; // a: [i32; 3]
     println!("a: {:?}", a);
 
     let a = ["foo", "bar"];
     println!("a has {} elements", a.len());
     println!("The first element of a is: {}", a[1]);

     // String
    let mut s = String::new();
    println!("s: {}", s);
 
     // Slices
     let a = [0, 1, 2, 3, 4];
     let middle = &a[1..4]; // A slice of a: just the elements 1, 2, and 3
     let complete = &a[..]; // A slice containing all of the elements in a
     println!("middle: {:?}", middle);
     println!("complete: {:?}", complete);
 
     // Tuples
     let x = (1, "hello");
     println!("x: {:?}", x);
 
     let x: (i32, &str) = (1, "hello");
     println!("x: {:?}", x);
 
     let overwatch = ("Tracer", 150, true);
 
     println!("Hero: {}", overwatch.0);
     println!("Health: {}", overwatch.1);
     println!("Is alive: {}", overwatch.2);
 
     // Functions
     fn foo(x: i32) -> i32 { x }
     let x: fn(i32) -> i32 = foo;
     println!("x: {}", x(42));
}