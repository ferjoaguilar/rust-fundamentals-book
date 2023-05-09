struct Person{
    name: String,
    age: u8
}
trait Greeter {
    fn greet(&self);
}

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old", self.name, self.age);
    }
}
    
fn trait_func() {

    let person: Person = Person {
        name: String::from("John"),
        age: 32
    };

    person.greet();
}