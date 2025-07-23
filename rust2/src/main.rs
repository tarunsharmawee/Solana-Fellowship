trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("tha name is {}, and the age is {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Tarun"),
        age: 18,
    };
    println!("{}", user.summarize());
}
