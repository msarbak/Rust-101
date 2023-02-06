struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
}

fn main() {
    let mehmet = Person { name: String::from("Mehmet"), age: 22 };

    println!("{}", mehmet.to_string());
}
