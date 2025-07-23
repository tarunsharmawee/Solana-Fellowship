struct User<'a> {
    name: &'a str,
}

fn main() {
    let user;
    {
        let name = String::from("Tarun");
        user = User { name: &name };
    }

    print!("{}", user.name)
}
