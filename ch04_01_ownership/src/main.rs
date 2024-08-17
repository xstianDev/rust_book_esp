//////////////////////////////
// 4.1 Ownership
// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
//////////////////////////////

fn stack_heap_scopes() {
    /* Conceptos importantes:
     * - Ownership: permite a Rust garantizar la seguridad de la memoria sin usar un garbage collector.
     * 
     * - Borrow checker: establece normas para realizar ownership. Si no se cumplen, da errores en compilación
     * 
     * - Stack: almacena capas (stack frame) para cada función que ejecuta. Cada stack frame guarda las variables
     *   locales de la función ejecutada. El tamaño del stack es fijo y se calcula en tiempo de compilación 
     *   (cada variable también debe tener un tamaño fijo). Las variables de los stack frames solo viven mientras
     *   el stack frame viva.
     * 
     * - Heap: puede cambiar de tamaño en ejecución. Los datos que almacena pueden tener tamaños dinámicos y 
     *   controlamos su tiempo de vida.
     * 
     *   Ejemplo: los strings pueden tener tamaño variable, por lo que se guardan en el heap en vez del stack.
     *   Añadir al stack es más rápido que asignar memoria en el heap, ya que este tiene que buscar un espacio libre.
     *   También es más rápido acceder datos en el stack que en el heap, ya que en este hay que seguir el puntero.
     */

    fn a() {
        let x: &str = "hello";                  // Heap:  save 'x'
        let y: i32 = 22;                        // Heap:  save 'y'
        b();                                    // Stack: push 'b'
        //                                      // Stack: pop  'b'
    }                                           // Heap:  del  'x', 'y'

    fn b() {
        let x: String = String::from("world");  // Heap:  save 'x' (diferente scope, diferente referencia)
    }                                           // Heap:  del  'x'

    a();                                        // Stack: push 'a'
}

fn memory_allocation() {
    /* Maneras de manejar la memoria:
     * 
     * Garbage collector:
     *   + Sin errores (aunque los gc pueden tener bugs)
     *   + Escritura del código más rápida
     *   - No tienes control sobre la memoria
     *   - Menor y menos predecible rendimiento en ejecución.
     *   - Tamaño de programa más grande
     * 
     * Manual:
     *   + Control sobre la memoria
     *   + Ejecución más rápida
     *   + Tamaño de programa más pequeño
     *   - Propenso a errores
     *   - Tiempo de escritura más lento
     * 
     * Ownership:
     *   + Control sobre la memoria
     *   + Sin errores (checks en tiempo de compilación) 
     *     (seguro por defecto, pero también tiene un modo no seguro)
     *   + Ejecución más rápida
     *   + Tamaño de programa más pequeño
     *   - Tiempo de escritura más lento que manejo manual
     *     (hay que cumplir las normas del borrow checker)
     */

    let s1 = String::from("hello");     // Ver Ch04_01
    let s2 = s1;                        // s2 es el nuevo owner del valor de s1 

    // Propiedades de string: ptr (pointer), len y capacity
    /* s2 no copia el valor de s1       (el heap buscaría un hueco para el nuevo valor)
    * s2 no hace shallow copy de s1    (puntero de s2 y s1 apuntando al mismo valor)
    * s2 hace un move y se invalida s1 (nuevo owner del valor)
    */

    // println!("{}, world!", s1);                 // borrow of moved value: `s1`

    let s3 = s2.clone();


    let x = 5;
    let y = x;  // Copia valor
    /* Rust usa "copy trait" para valores simples (ints, bools, chars) 
    * Son valores que se copian, no se mueven
    */
}

fn ownership() {
    /* Reglas del ownership: 
     * 1. Cada valor tiene una variable que es su owner.
     * 2. Solo puede haber un owner a la vez.
     * 3. Cuando el owner se sale del scope, el valor se elimina.
     */

    /** Toma ownership del valor al pasarlo como parámetro */
    fn takes_ownership(text: String) {
        println!("{}", text);
    }
    
    /** Devuelve ownership del valor */
    fn gives_ownership() -> String {
        String::from("hello")
    }
    
    /** Copia el valor sin tomar ownership porque es simple */
    fn makes_copy(num: i32) {
        println!("{}", num);
    }
    
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);          // borrow of moved value: `s`
    
    let owner = gives_ownership();
    println!("{}", owner);
    
    let n = 123;
    makes_copy(n);
    println!("{}", n);
}

fn main() {
    stack_heap_scopes();
    memory_allocation();
    ownership();
}
