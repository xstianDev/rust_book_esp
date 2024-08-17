//////////////////////////////
// 16.1. Threads (hilos)
// https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html
// https://doc.rust-lang.org/stable/book/ch16-01-threads.html
//////////////////////////////

use std::{thread, time::Duration};

fn threads_and_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Bloquea los hilos de main para que esperen a los de spawn
    handle.join().unwrap();
}

fn move_closures_with_threads() {
    let v = vec![1, 2, 3];

    // Con "move" aseguramos que los valores dentro del closure no hagan borrow,
    // sino que el closure tenga ownership de los valores
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main() {
    threads_and_join();
    move_closures_with_threads();
}
