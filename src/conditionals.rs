fn conditionals(){
    let x = 5;
    if x == 5 {
     println!("X is five!");
    }
 
    let x = 6;
    if x == 6 {
     println!("X is six!");
    } else {
     println!("X is not six :(");
    }
 
    let x = 5;
 
    let y = if x == 5{
     10
    } else {
     15
    };
 
    println!("y: {}", y);
 
    let x = 5;
    let y = if x == 5 { 10 } else { 15 };
    println!("y: {}", y);
}