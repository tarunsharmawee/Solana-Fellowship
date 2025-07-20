use std::collections::HashMap;

fn group_value_by_key(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec{
        hm.insert(key, value);
    }
    return hm; 
}
 
 fn main(){
    let input_vec = vec![(String::from("Tarun"), 18), (String::from("Tarun"), 18)];
    let hm = group_value_by_key(input_vec);
    print!("{:?}", hm)
}