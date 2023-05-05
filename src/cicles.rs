fn cicles(){
    loop {
        println!("Hello, world!");
        break;
    }

    let mut counter = 5;
    let mut completed = false;

    while !completed {
        counter += counter - 3;

        println!("x = {}", counter);

        if counter % 5 == 0 {
            completed = true;
        }
    }

    for x in 0..10 {
        println!("x = {}", x);
    }

    // Show to times iterate over cicle
    for (i, j) in (5..10).enumerate(){
        println!("i = {} and j = {}", i, j);
    }
}