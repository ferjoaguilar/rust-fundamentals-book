fn vectors(){
    let v = vec![1, 2, 3, 4, 5];
    println!("The third element of v is {}", v[2]);

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v{
        println!("Rereference to {}", i);
    }

    for i in &mut v {
        println!("Reference to mutable {}", i);
    }

    for i in v {
        println!("Take ownership of {}", i);
    }
}