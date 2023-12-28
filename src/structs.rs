struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
    // fn create(name: String, age: u8) -> Person {
    //     Person::new(name, age)
    // }

    fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }

    fn die(self) {
        println!("{} is dead.", self.name);
    }

    fn die_and_comeback(self) -> Person {
        println!("{} is dead.", self.name);
        self
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{} is dead.", self.name);
    }
}

pub fn use_structs() {
    let p = Person::new("Giovanni".to_owned(), 44);
    p.say_hello(); // = Person::say_hello(&p);
                   // p.die();
    let other_person = p.die_and_comeback();
    other_person.say_hello();
    other_person.die();

    println!("Before creating person");
    {
        let other = Person::new("Other".to_owned(), 22);
        other.say_hello();
    }
    // other.say_hello();
    println!("After person is out of scope");
    let money = Money(3, 2);
    println!("Money {:?}", money);
}

#[derive(Debug)]
struct Money(u32, u32);
