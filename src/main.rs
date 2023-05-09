mod english;

/* use english::greetings;
use english::goodbyes; */

//use english::{greetings, goodbyes};
use english::{greetings as saludos, goodbyes as despedidas};


fn main() {
    println!("English func {}", saludos::hello());
    println!("English func {}", despedidas::goodbye());
}