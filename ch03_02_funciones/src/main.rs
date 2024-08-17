//////////////////////////////
// 3.2. Funciones
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
//////////////////////////////

use std::any::type_name;

// Devuelve '()' cuando no se devuelve ningún tipo de valor.
// El tipo '()' se llama 'unit'. Representa una tupla vacía.
fn my_func() {
    println!("my_func");
}

// fn [nombre](params) -> T { ... }
fn sum(x: i32, y: i32) -> i32 {
    println!("x: {x}");
    println!("y: {y}");

    let res = x + y;
    return res;
}

// Se puede devolver sin escribir return. Este valor va sin ';'
fn div(x: f32, y: f32) -> f32 {
    println!("x: {x}");
    println!("y: {y}");

    let res = x / y;
    res
}

// Se puede devolver directamente la operación
fn mult(x: i8, y: i8) -> i8 {
    println!("x: {x}");
    println!("y: {y}");

    x * y
}

// Función para ver el tipo de un dato
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


/* - Statement: realiza una acción pero no devuelve nada.
 * - Expression: realiza una acción y devuelve un valor.
 */
fn main() {
    my_func();
    
    let i = my_func();
    println!("Tipo: {}", type_of(i));

    let n = sum(3, 5);
    println!("Sum: {}", n);
    
    let n = div(20.0, 7.0);
    println!("Div: {}", n);
    
    let n = mult(2, 3);
    println!("Mult: {}", n);
}