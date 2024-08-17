//////////////////////////////
// 4. Funciones
//////////////////////////////
use std::any::type_name;

fn main() {
    my_func();
    
    let i = my_func();
    println!("Tipo: {}", type_of(i));

    let n = suma(3, 5);
    println!("Sum: {}", n);
}

// Implícitamente devuelve () o tipo unit, que se usa cuando no se devuelve ningún tipo de valor.
fn my_func() {
    println!("my_func");
}

// fn [nombre](params) -> T { ... }
fn suma(x: i32, y: i32) -> i32 {
    println!("x: {} | y: {}", x, y);
    let sum = x + y;
    return sum;
}

// Función para ver el tipo de un dato
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}