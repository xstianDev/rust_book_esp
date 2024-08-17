//////////////////////////////
// 14.1. Crates.IO
// https://doc.rust-lang.org/stable/book/ch14-00-more-about-cargo.html
// https://doc.rust-lang.org/stable/book/ch14-01-release-profiles.html
// https://doc.rust-lang.org/cargo/reference/profiles.html
// https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html
//////////////////////////////

/* Release profiles: permiten configurar como se compila el código.
 * Cargo tiene dos perfiles principales: dev y release.
 * Cada uno define valores por defecto para su propósito específico.
 * 
 * cargo build: por defecto, su perfil es dev. Usa una versión no optimizada y con debuginfo.
 * cargo build --release: optimiza la compilación.
 * 
 * Para cambiar la configuración, vamos a Cargo.toml
 */
//

// Comentario que documenta el item que contiene los comentarios de documentación (///)
// En este caso, comenta el crate
//! # My Crate
//! `my_Crate` is a collection of utilities to make performing certain
//! calculations more convenient

// Comentario de documentación (///)
// Con "cargo doc --open" genera un HTML y lo abre en el navegador. 
// Con "cargo test" se puede probar el test de la documentación

/// Adds one to a the number given
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

