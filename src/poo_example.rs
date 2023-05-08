struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name: name, age: age }
    }

    fn say_name(&self) {
        println!("Hello my name is {} and my age is {}", self.name, self.age);
    }
}

fn poo_impl() {
    let person = Person::new("John".to_string(), 25);
    person.say_name();
}