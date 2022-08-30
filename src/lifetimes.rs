#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: Box<u8>,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u8) -> Person {
        Person {
            name,
            age: Box::new(age),
        }
    }

    fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

pub fn use_lifetimes() {
    let p = Person::new("Giovanni", 44);
    p.say_hello();
    {
        let Person { name, ref age } = p;
        println!("{} is {} years old.", name, age);
    }
    println!("Person is {:?}", p);
}

// fn use_structs_incorrectly() -> Person {
//     let name = String::from("Giovanni");
//     let p = Person::new(&name, 44);
//     p
// }
