//////////////////////////////
// 15.5. RefCell Pointer
// https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html
//////////////////////////////

mod mock;

fn rc_and_refcell() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

fn main() {
    /* Interior mutability: patrón de diseño en Rust que permite mutar datos incluso cuando hay
     * referencias inmutables a esos datos, lo cual suele estar prohibido por las reglas de borrowing.
     * 
     * Este patrón usa código inseguro dentro de una estructura de datos para poder saltarse las reglas
     * sobre mutación y borrowing. El código inseguro no se comprueba en compilación.
     * Aún así, se pueden aplicar las reglas en ejecución.
     */

    /* RefCell: representa ownership único sobre sus datos, como Box. La diferencia entre ambos es que
     * Box realiza las reglas de borrowing en compilación, mientras que RefCell lo hace en ejecución.
     * 
     * En Rust, por defecto se aplican las reglas en compilación para captar errores antes de que
     * sucedan durante la ejecución.
     * 
     * RefCell vs. otros punteros:
     * - RefCell<T> solo permite un owner, Rc<T> permite múltiples owners.
     * 
     * - RefCell<T> permite borrows mutables o inmutables comprobados en ejecución.
     *   Rc<T> solo permite borrows inmutables comprobados en compilación.
     *   Box<T> permite borrows mutables o inmutables comprobados en compilación.
     * 
     * - RefCell<T> permite borrows mutables comprobados en ejecución. Por ello se puede mutar el valor de 
     *   RefCell incluso cuando es inmutable.
     */

    /*
    let a = 5;
    let b = &mut a;     // cannot mutate immutable variable `a`
    
    let mut c = 10;
    let d = &c;
    *d = 20;    // cannot assign to `*d`, which is behind a `&` reference; `d` is a `&` reference, so the data it refers to cannot be written
    */

    rc_and_refcell();
}
