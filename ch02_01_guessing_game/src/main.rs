//////////////////////////////
// 2. Programming a Guessing Game
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//////////////////////////////

use std::io;                // Librería estándar (std). Toma input del usuario (io). 
use std::cmp::Ordering;     // Librería estándar (std). Se usa para comparar valores (cmp)
use colored::Colorize;      // Librería externa. Da color al texto.
use rand::Rng;              // Librería externa. Genera número aleatorios.

/* Instalar una crate con cargo
 * - Abrir Cargo.toml
 * - Nombre de la crate en [dependencies]
 * - Instalar con "cargo build"
 */ 

/* El compilador da error si no se usa una variable. 
 * Para que la ignore, se pone un '_' antes del nombre.
 * 
 * Rust detecta los tipos automáticamente. 
 * Se puede inferir el tipo de manera implícita usando '[var]: [type]'. 
 * 
 * Por defecto, las variables en Rust son inmutables.
 * mut: palabra reservada para declarar una variable mutable.
 *         
 * Shadowing: declarar la variable otra vez con un nombre ya existente.
 * match: palabra reservada que hace como 'switch' sin usar el 'case'.
 */
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    // print con un placeholder para la variable.
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");

        // Función asociativa de String (en otros lenguajes, 'static'). 
        // String es el tipo de dato y new() es la función que lo crea.
        let mut guess = String::new();

        // &mut guess: referencia a una variable mutable.
        // Sirve como buffer y le añade contenido sin tomar ownership.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,    // Devuelve el valor.
            Err(_) => continue,     // '_' atrapa todos los valores.
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}