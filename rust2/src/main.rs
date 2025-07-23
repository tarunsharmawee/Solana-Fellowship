fn main() {
    let ans;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest(&str1, &str2); // this is giving error coz it's lifetime ends with the scope so u cant use it afterwardsgit a
    }
    print!("longer string is : {ans}")
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str2.len() > str2.len() {
        return str2;
    } else {
        return str2;
    }
}
