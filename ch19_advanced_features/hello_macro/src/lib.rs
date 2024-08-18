/* Hay una convención para nombrar crates y macro crates.
 * Si la crate es una custom derived macro, su nombre será: [crate_name] + _derive
 * 
 * Cada crate tiene que ser publicada por separado y el código que usa estas crates debe introducir
 * todas las crates en su scope. Para ello hay que cambiar el archivo Cargo.toml de la crate _derive.
 */

pub trait HelloMacro {
    fn hello_macro();
}