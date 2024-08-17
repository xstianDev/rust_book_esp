//////////////////////////////
// 15.1. Box Pointer
// https://doc.rust-lang.org/stable/book/ch15-00-smart-pointers.html
// https://doc.rust-lang.org/stable/book/ch15-01-box.html
//////////////////////////////

fn box_smart_pointer() {
    /* Pointers: un puntero es una variable que guarda una dirección de memoria que apunta a otros datos en memoria.
     * El puntero más común en Rust es una referencia, que toma prestado el valor al que apunta sin tener ownership.
     * Las referencias no tienen muchas capacidades.
     * 
     * Smart pointers: estructuras de datos que actúan como punteros que tienen metadatos y capacidades. Por ejemplo,
     * un smart pointer que cuenta referencias, lo que permite que un dato tenga varios owners y llevar la cuenta de 
     * ellos para limpiar el dato una vez no sea usado.
     * En muchos casos, los smart pointers son owners de los datos a los que apuntan, no como las referencias.
     * 
     * String y Vec son smart pointers. Son owners de los datos a los que apuntan, permiten manipularlos y guardan
     * metadata como la capacidad y otras capacidades, como asegurar que un String sea UTF-8.
     * 
     * Los smart pointers se suelen implementar como structs que implementan los traits deref y drop.
     * - deref: permite que instancias del puntero sean tratadas como referencias para escribir código que funcione
     *   con referencias o smart pointers.
     * - drop: personaliza el código que se ejecuta cuando una instancia del puntero sale del scope.
     * 
     * Los smart pointers son un patrón de diseño que muchas librerías implementan.
     */

    /* Box se usa cuando:
     * - Un tipo cuyo tamaño exacto no se conoce en compilación y queremos usar un valor de ese tipo en un contexto
     *   donde necesitamos saber el tamaño exacto.
     * - Tenemos muchos datos y queremos transferir el ownership asegurando que no se copia.
     * - Eres owner de un valor y solo importa que implementa un trait específico (trait object).
     */

    // El valor pasado a new() es el que almacenamos en el heap, y en el stack se guarda una dirección de memoria al valor
    let b = Box::new(5);
    println!("b = {}", b);
}

fn box_recursive() {
    // Indirección con Box para que el tamaño de List no sea infinito. Guarda un puntero a List, no una nueva instancia
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // Se guardan 
    let list = Cons(
        1, Box::new(Cons(
            2, Box::new(Cons(
                3, Box::new(Nil)
            ))
        ))
    );
}

fn main() {
    box_smart_pointer();
    box_recursive();
}
