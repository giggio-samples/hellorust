struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

pub fn use_structs() {
    let p = Person::new("Giovanni".to_owned(), 44);
    p.say_hello();
}
