//////////////////////////////
// 8.1. Vector
// https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html
// https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
//////////////////////////////

/* Colección: permite guardar varios valores, pero al contrario de los arrays o tuplas,
 * se alojan en el heap (pueden crecer).
 */

fn vectors() {
    /* Vectores: arrays que solo almacenan un tipo de dato.
     * Al salir del scope, el vector y sus elementos son droppeados del heap.
     * 
     * Si se intenta acceder a un índice que no existe en el vector Rust lanza un runtime error,
     * pero si fuese un array lanzaría un compile error. Esto es porque el tamaño del vector es
     * desconocido en tiempo de ejecución.
     */

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // macro vec!: inicializa el vector con unos valores.
    let mut v = vec![1, 2, 3, 4, 5];

    // Acceso a elementos
    let third = &v[2];
    // v.push(6);   // Error: cambia la referencia de v[2] 
    println!("The third element is {third}");

    // get
    match v.get(9) {
        Some(fourth) => println!("The tenth element is {fourth}"),
        None => println!("There is no tenth element."),
    }

    // for
    for i in &mut v {
        // dereferencing operator
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }
}


fn cell_vector() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Pepe")),
        SpreadsheetCell::Float(12.34),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer"),
    }
}

fn main() {
    vectors();
    cell_vector();
}
