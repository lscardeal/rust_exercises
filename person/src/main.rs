#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn birthday(self) -> Self {
        Self { name: self.name, age: self.age + 1 }
    }
}

fn main() {
    let person: Person = Person { name: String::from("Ana"), age: 10 };
    let person = person.birthday();
    print!("{:#?}", person);
}
