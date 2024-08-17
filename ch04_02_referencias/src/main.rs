//////////////////////////////
// 4.2. Referencias
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
//////////////////////////////

fn references() {
    /* Referencias: inmutables por defecto. Toman un valor pero no su ownership (borrowing). */
    
    /** Toma ownership de un valor y lo devuelve  */
    fn calc_length(s: String) -> (String, usize) {
        let len = s.len();
        (s, len)
    }

    // Las referencias evitan tomar ownership si no es necesario
    fn get_len(s: &String) -> usize {
        s.len()
    }

    fn change(s: &mut String) {
        s.push_str(", world")
    }

    let s = String::from("hello");
    let len = get_len(&s);  // Borrowing: toma la referencia de un valor pero no su ownership
    println!("La longitud de '{}' es {}", s, len);
    
    let mut msg = s.clone();
    change(&mut msg);
}

fn reference_rules() {
    /* Reglas de las referencias:
     * - Solo puede haber una referencia mutable a un valor específico dentro de un scope específico 
     *   o cualquier número de referencias inmutables.
     * - Las referencias siempre deben ser válidas. 
     */

     /* No se permite más de una referencia mutable a un valor dentro del mismo scope
    * para evitar data race (cuando dos hilos o más acceden a un dato).
    * Así se previene que un puntero lea información mientras otro la escribe y no
    * se corrompa la información.
    */
    /*
    let mut a = String::from("hello");
    let b: &mut String = &mut a;
    let c: &mut String = &mut a;   // cannot borrow `a` as mutable more than once at a time
    println!("{}, {}", b, c);    
    */
    
    // No se permite tener una referencia mutable a un valor cuya referencia es inmutable. 
    /*
    let d = &mut a;     // cannot borrow `a` as immutable because it is also borrowed as mutable
    println!("{}, {}", a, d);
    */
    
    // Sí se permite más de una referencia a valores inmutables ya que no van a cambiar.
    let a = String::from("hello");
    let b = &a;
    let c = &a;
    println!("{}, {}", b, c);

    // Después del scope de la variable sí es posible evitar la race condition
    let mut d = String::from("hello");

    let e: &String = &d;    // Comienza el scope de 'e'
    println!("{}", e);      // Acaba el scope de 'e'
    
    let f: &mut String = &mut d;    // 'f' no altera 'e' porque su scope ha acabado, aunque ambos referencian a 'd'
    println!("{}", d);
}

fn dangling() {
    /* Dangling: cuando una referencia apunta a datos inválidos */ 
    
    /*
    /* Cuando acaba el scope de la función (al devolver el valor) se desaloja el valor de 's' del heap,
     * haciendo que la referencia devuelta no apunte a los datos correctos.
     */
    fn dangle() -> &String {
        // missing lifetime specifier
        // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
        // consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`: `'static `
        let s = String::from("hello");
        &s
    }
    */
}

fn main() {
    references();
    reference_rules();
    dangling();
}
