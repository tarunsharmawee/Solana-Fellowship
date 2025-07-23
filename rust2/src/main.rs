trait Summary {
    fn summarize(&self) -> String{
        return String::from("hi there");
    }
}

trait  Fix{
    fn fix(&self) -> String{
        return String::from("HI there 2");
    }
}

struct User {
    name: String,
    age: u32,
} 

impl Summary for User{}
impl Fix for User{}

fn main() {
    let user = User {
        name: String::from("Tarun"),
        age: 18,
    };
    notify(user);
}

fn notify<T: Summary + Fix>(u: T){
    println!("{}",u.summarize());
}