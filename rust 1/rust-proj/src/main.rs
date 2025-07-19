struct Result<T, B> {
    x: T,
    y: B,
    z: B,
}

fn main() {
    let result = Result { x: 5, y: "Hello", z: "World" };
    println!("Result x: {}, y: {}, z: {}", result.x, result.y, result.z);
}