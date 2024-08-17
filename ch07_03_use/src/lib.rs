//////////////////////////////
// 7.3. Use
// https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
//////////////////////////////


// Nested paths
use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};

// Glob operator
use std::ascii::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute path + exportación del módulo
pub use crate::front_of_house::hosting;

// Relative path
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    /* La manera de traer funciones dentro del scope es trayendo el módulo padre al scope.
     * Así se minimiza el path y queda claro que la función no es local sino que está en 
     * otro módulo. Con structs, enums... también se especifica el path completo.
     * 
     * Como excepción, cuando tenemos dos items que se llaman igual pero de dos módulos
     * diferentes, lo que se hace es traer el módulo padre dentro del scope.
     */
    hosting::add_to_waitlist();
}


// Excepción
use std::fmt::Result;
use std::io::Result as IoResult;

fn func1() -> Result {
    Ok(())
}

fn func2() -> IoResult<()> {
    Ok(())
}
