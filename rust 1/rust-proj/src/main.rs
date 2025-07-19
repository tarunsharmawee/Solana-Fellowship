struct User {
    name: String,
    age: u32,
    active : bool,
}

fn main(){
    let user = User {
        name: String::from("Tarun"),
        age: 18,
        active: true
    };
    print!("user is {} Years old and is {}ly active", user.age, user.active)
}