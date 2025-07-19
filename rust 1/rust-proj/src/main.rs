use std::fs;

enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content) =>{
            println!("File content: {}", content)
        },
        Err(err) =>{
            println!("error: {}", err)
        }
    }
}