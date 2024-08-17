//////////////////////////////
// 16.3. Sharing State
// https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html
// https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html
//////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

fn mutex_api() {
    /* Mutex: mutual exclusion. Cuando hay un dato y solo un hilo lo puede acceder cuando sea.
     * Para ello se usa el lock, una estructura que no permite el acceso a un dato cuando ya hay
     * un hilo que tiene acceso al lock. Cuando el hilo acaba de usar el dato, libera el lock.
     * Mutex usa interior mutability y puede provocar deadlocks.
     * 
     * Los mutex suelen ser complicados:
     * - Hay que tener un lock antes de acceder al dato.
     * - Hay que liberar el lock al acabar.
     * 
     * El sistema de tipos y las reglas de ownership de Rust hacen que no sea complicado este proceso. 
     */

    let m = Mutex::new(8);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // El `Drop` trait de `MutexGuard` hace que se libere el lock.
    }

    println!("m = {:?}", m);
}

fn mutex_threads() {
    // Arc: asynchronous Rc. Es como Rc pero con seguridad para hilos.
    // La relación entre Mutex y Arc es la misma que la de RefCell y Rc.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut n = counter.lock().unwrap();
            *n += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // Rust no tiene muchas funcionalidades de concurrencia, pero tiene los traits 
    // `Send` y `Sync` para crear nuestras propias funcionalidades.
}

fn main() {
    mutex_api();
    mutex_threads();
}
