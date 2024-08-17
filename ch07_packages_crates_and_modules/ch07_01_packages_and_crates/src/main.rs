//////////////////////////////
// 7.1. Packages && Crates
// https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
// https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html
//////////////////////////////

/* Packages: característica de Cargo. Permite construir, testear y compartir crates.
 * Al hacer 'cargo new' estamos definiendo un nuevo package.
 * 
 * Reglas:
 * - Debe tener al menos una crate
 * - Puede tener 0 library crates o 1 library crates.
 * - Puede tener cualquier número de binary crates.
 * - Las binary crates van dentro de src/bin
 */

/* Crates: árbol de módulos que produce una librería o ejecutable.
 * Hay dos tipos de crate: binary o library. La diferencia entre ambas es que las binary 
 * tienen un main().
 * Por defecto, al generar el paquete con 'cargo new' ya tenemos una binary crate: main.rs
 * 
 * El archivo main.rs es denominado 'crate root', donde el compilador de Rust comienza a construir
 * las crates. También construye el módulo main de la crate.
 * 
 * Si se crea un archivo 'lib.rs' en la raíz del directorio src, Rust lo interpretará como una
 * library crate. Con el mismo nombre del paquete y lib.rs será la 'crate root'.
 */

/* Modules y use: permiten controlar la organización, scope y privacidad de los paths
 * Paths: una manera de nombrar un item (componente de una crate), como un struct, función o módulo
 */
fn main() {

}
