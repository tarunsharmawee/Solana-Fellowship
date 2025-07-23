trait Summary {
    fn summarize(&self) -> String{
        return String::from("hi there");
    }
}

struct User {
    name: String,
    age: u32,
}

struct Fix;
impl Summary for User {}
impl Summary for Fix{}
impl Summary for String{}

fn main() {
    let user = User {
        name: String::from("Tarun"),
        age: 18,
    };
    notify(user);
}

fn notify(u: impl Summary){
    println!("{}",u.summarize());
}