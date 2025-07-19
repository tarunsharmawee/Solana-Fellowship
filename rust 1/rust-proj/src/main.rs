fn main() {
    let mut s1 = String::from("Hello ");
    update_str(&mut s1);
    print!("{}", s1);
}

fn update_str(s: &mut String) {
    s.push_str(" World");
}   