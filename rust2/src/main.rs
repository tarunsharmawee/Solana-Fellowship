use std::{
    collections::btree_map::Values,
    sync::mpsc,
    thread::{self, spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 1000000..(i + 1 * 1000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap()
        });
    }
    drop(tx);

    let mut final_sum = 0;
    for val in rx{
        final_sum = final_sum + val;
    }
    println!("{}", final_sum)
}
