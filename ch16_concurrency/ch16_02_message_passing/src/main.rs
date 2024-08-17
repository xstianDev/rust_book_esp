//////////////////////////////
// 16.2. Message Passing
// https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html
//////////////////////////////

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn messages_and_channels() {
    /* Para asegurar la concurrencia segura se suele usar el paso de mensaje,
     * donde hilos o actores pasan mensajes entre sí que contienen datos.
     * 
     * Según Rust: Do not communicate by sharing memory; instead, share memory by communicating.
     * 
     * Para pasar mensaje se usan canales (channel). Los canales tienen dos mitades: transmisor y receptor.
     * En caso de los hilos, el transmisor es el hilo que manda datos y el receptor es quién los recibe.
     * El canal se cierra cuando el transmisor o el receptor son eliminados.
     * 
     * mpsc: multi-producer, single-consumer
     */ 

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn sending_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        } 
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        } 
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        } 
    });

    for received in rx {
        println!("Got: {}", received);
    }

}

fn main() {
    messages_and_channels();
    sending_multiple_values();
    multiple_producers();
}
