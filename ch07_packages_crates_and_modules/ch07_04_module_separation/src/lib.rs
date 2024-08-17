//////////////////////////////
// 7.4. Module Separation
// https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html
//////////////////////////////

// importa un módulo
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}