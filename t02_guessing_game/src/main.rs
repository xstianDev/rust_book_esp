//////////////////////////////
// Ch 2. Programming a Guessing Game
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//////////////////////////////

//////////////////////////////
// 2. Guessing game
//////////////////////////////

// std::io : librería estándar de Rust (std) que toma input del usuario (io).
// std::cmp::Ordering : librería estándar de Rust (std) que se usa para comparar valores (cmp)
// colored::Colorized : librería externa que da color al texto.
// rand::Rng : librería externa que genera número aleatorios.
use std::io;
use std::cmp::Ordering;
use colored::Colorize;
use rand::Rng;

// cargo new [nombre]: crear un proyecto.
// cargo init: crea un proyecto en un directorio existente.
/* Para instalar un paquete externo con cargo, hay que ir a Cargo.toml y escribir el nombre
 * del paquete debajo de [dependencies]. Después, se hace "cargo build" para instalarlo.
 * rand = "0.5.5"
 */ 

fn main() {
    println!("Guess the number!");
    
    /* - rand::thread_rng() -> ThreadRng: inicializa un hilo que genera un número aleatorio. 
     *   Se supone que se usa en cadena. Devuelve una referencia ThreadRng.
     * - Rng.gen_range(&mut self, low: T, high: T) -> T: devuelve un número aleatorio entre 
     *   dos números de tipo T (int, float...). En la generación incluye low pero excluye min. 
     */
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    // print con un placeholder para la variable.
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess!");
        
        // Si no se usa la variable, el compilador dará error. 
        // Para que ignore esto, se puede poner un "_" antes del nombre de la variable.
        
        // Rust detecta el tipo de variable por defecto, pero se le puede inferir el tipo 
        // de manera implícita usando :[tipo]. 
        
        // Por defecto, las variables en Rust son inmutables.
        // mut: palabra reservada para declarar una variable mutable.

        /* String::new()
         * - String: tipo de dato de la librería estándar de Rust.
         * - new(): función de String que crea un nuevo String.
         * - String::new() : función asociativa de String. En otros lenguajes, funciones "static". 
         */
        let mut guess = String::new();
        
        /* - io::stdin(): maneja el input del usuario. Devuelve una referencia compartida a un 
         *   buffer global ("devuelve un objeto Stdin").
         * - Stdin::read_line(&self, buf: &mut String) -> io::Result<usize> : toma como parámetro una 
         *   referencia a una variable mutable que sirve como buffer y añade (append) el contenido a ella 
         *   sin tomar pertenencia sobre ella (ownership). 
         *   Devuelve una enumeración Result<T, E> con dos variantes:
         *   • Ok(T): representa éxito y contiene un valor.
         *   • Err(E): representa error y contiene un valor error.
         * - Result::expect(self, msg: &str) -> T: en caso de tomar un valor Ok, devuelve este. En caso de recibir 
         *   un valor Err (panic), muestra un mensaje. No se recomienda su uso porque no previene el error.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /* - trim(&self) -> &str: quita los espacios al comienzo y al final del string.
         * - parse<F: FromStr>(&self) -> Result<F, F::Err>: convierte el String a otro tipo de dato. Al ser una función
         *   tan general, hay que decirle a qué tipo de dato va a convertir. Devuelve un Result<T, E>.
         */
        // Shadowing: declarar la variable otra vez con un nombre ya existente.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_): la _ significa que va a atrapar todos los valores.
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        /* - String.cmp(&self, other: &Self) -> core::cmp::Ordering : compara la referencia de sí mismo con la de otro 
         *   valor y devuelve un Ordering.
         * - colored::Colorized.red() -> ColoredString: convierte un texto a color rojo.
         * - colored::Colorized.green() -> ColoredString: convierte un texto a color verde.
         */
        // match: palabra reservada que hace como el switch de Java.
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