//////////////////////////////
// 7.2. Modules
// https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
//////////////////////////////

/* Archivo y código generado por defecto con 'cargo new --lib ch07_02_restaurant'
 * Al crear un paquete con --lib se crear un library crate en vez de binary.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

/* Module: definido con mod [nombre].
 * Los módulos padre no pueden ver el contenido de los hijos, pero estos sí 
 * pueden ver los de su padre. Por defecto, los hijos son privados.
 */ 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // private
        fn sit_at_table() {}
    }
    
    // private
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


// Paths
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/* Estructura para el compilador:

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

crate es un módulo que se crea por defecto para el crate root (lib.rs).
Dentro está el módulo front_of_house, dentro del cual están hosting y serving
con sus funciones.
*/


// super
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super: referencia al módulo padre (crate)
        super::serve_order();
    }

    fn cook_order() {}
}


// privacy rules in items
mod house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_breakfast() {
    /* Aquí dentro solo se puede declarar breakfast mediante la función summer, ya que
     * Breakfast tiene un campo privado (seasonal_fruit)
     */
    let mut meal = house::Breakfast::summer("Rye");

    // Las propiedades de los enums se vuelven públicas si el enum es público
    let order1 = house::Appetizer::Soup;
    let order2 = house::Appetizer::Salad;
}