fn main () {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}",vec);
    let even_val = even_val_vector(vec);
    println!("{:?}",even_val);
    
}
fn even_val_vector(vec: Vec<usize>)-> Vec<usize>{
    let mut new_vec = Vec::new();

    for i in vec[0]..vec.len() + 1{
        if i % 2 == 0 {
            new_vec.push(i);
        }
    }
    return new_vec;
}   