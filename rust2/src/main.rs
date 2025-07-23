use std::{collections::btree_map::Values, sync::mpsc, thread::{self, spawn}};

fn main() {
    let (tx, rx) = mpsc::channel();
    spawn(move|| {
        tx.send(String::from("Hello World!")).unwrap()
    });

    let value = rx.recv().unwrap();
    println!("{}", value)
}
    