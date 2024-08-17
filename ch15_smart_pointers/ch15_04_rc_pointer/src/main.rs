//////////////////////////////
// 15.4. Rc Pointer
// https://doc.rust-lang.org/stable/book/ch15-04-rc.html
//////////////////////////////

fn reference_counter_pointer() {
    /* Reference counting smart pointer: se usa cuando queremos asignar un valor en el heap y tener
     * varias partes de nuestro programa leyendo ese valor y no sabemos qué parte va a acabar la 
     * última en tiempo de compilación.
     * 
     * Si supiéramos qué parte va a usar los datos en último lugar, podríamos hacer que esa parte 
     * fuese owner del valor.
     * 
     * El smart pointer Rc<T> solo vale para programas con un solo hilo.
     */
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(5, Rc::clone(&a));
    let c = Cons(5, Rc::clone(&a));
}

fn reference_count() {
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after a: {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after b: {}", Rc::strong_count(&a));
    {
        let c = Cons(3, Rc::clone(&a));
        println!("count after c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
}

fn main() {
    reference_counter_pointer();
    reference_count();
}
