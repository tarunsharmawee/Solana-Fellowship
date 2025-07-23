use std::thread;

fn main(){
    let handle = thread::spawn(||{
        for i in 0..5{
            println!("hi from spawned thread {i}")
        }
    });
    for i in 0..5 {
        println!("hi from main thread {i}")
    }
    handle.join();
}