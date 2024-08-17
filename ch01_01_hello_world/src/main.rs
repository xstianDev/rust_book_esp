//////////////////////////////
// Ch 1. Getting started
// https://doc.rust-lang.org/book/ch01-00-getting-started.html
// https://doc.rust-lang.org/book/ch01-01-installation.html
// https://doc.rust-lang.org/book/ch01-02-hello-world.html
// https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
// https://doc.rust-lang.org/book/ch03-04-comments.html
//////////////////////////////

/* Rust es un lenguaje compilado.
 * - rustc [file]: compila un archivo. Genera el ejecutable del programa con el mismo nombre. 
 * - cargo build: compila un proyecto. Genera el directorio "target" y el archivo "Cargo.lock".
 */

// Solo puede haber una función main por proyecto.
fn main() {
    // Función marco
    println!("Hello Rust!");
}