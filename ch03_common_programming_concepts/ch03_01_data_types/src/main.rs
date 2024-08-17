//////////////////////////////
// Ch 3.1. Common Programming Concepts
// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
// https://doc.rust-lang.org/book/ch03-02-data-types.html
//////////////////////////////

fn concepts() {
    // Variables inmutables por defecto
    let x = 5;
    println!("x: {}", x);
    // x = 6;  // cannot mutate immutable variable `x`    
    
    // Esta variable hace 'shadowing' a la anterior
    let x = "six";
    println!("x: {} (shadowing)", x);

    /* Shadowing:
    * - Conserva la mutabilidad.
    * - Permite cambiar el tipo.
    */

    const _NUM: u32 = 123456;       // Ignorado por la '_'
    // const mut NUM: u32 = 123456;    // Error: las constantes no pueden ser mutables.

    /* inmutables vs. constantes:
    * - Las inmutables no necesitan especificar su tipo, mientras que las constantes sí.
    * - Las inmutables pueden recibir cualquier valor, pero las constantes solo pueden tener un valor ya definido.
    *   Por ejemplo, una inmutable puede recibir un valor devuelto por una función, pero las constantes no pueden
    *   recibir un valor calculado en tiempo de ejecución.
    */
}

fn simple_types() {
    // Integers
    /* Según su tamaño, pueden ser de 8, 16, 32 (por defecto), 64 o 128 bit y 
     * tener signo (ej.: i8) o no tener signo (ej.: u8).
     * 
     * Hay un tipo especial "arch" que depende en la arquitectura, 
     * normalmente de 32 o 64 bits. Puede ser "isize" o "usize".
     */
    //

    // Los números grandes pueden llevar _ para distinguir mejor los dígitos.
    let a = 98_123;         // Decimal (i32)
    let b = 0xff;           // Hex     (i32)
    let c = 0o77;           // Octal   (i32)
    let d = 0b1111_0000;    // Binario (i32)
    let e = b'A';            // Byte    (u8)

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);


    /* En caso de sobrepasar el valor máximo, Rust entra en pánico en debug, mientras que 
     * en lanzamiento realizará "two's complement wrapping", es decir, que números que 
     * sobrepasan el valor máximo volverán al número mínimo (ej.: 256 -> 0, 257 -> 1...).
     */
    let f: u8 = 255;
    println!("f: {}", f);
    // println!("{}", f + 1);  // Pánico porque es debug


    // Floating-point numbers (f32 o f64)
    let g: f32 = 2.0;   // f32
    let h: f64 = 3.0;   // f64
    let i = 4.0;   // f64 (default)

    println!("g: {}", g);
    println!("h: {}", h);
    println!("i: {}", i);

    println!("+: {}", 5 + 10);
    println!("-: {}", 95.5 - 4.3);
    println!("*: {}", 4 * 30);
    println!("/: {}", 56.7 / 32.2);
    println!("%: {}", 43 % 5);


    // bool
    let t = true;
    let f = false;

    println!("bool t: {}", t);
    println!("bool f: {}", f);


    // char: carácter Unicode (comillas simples).
    let ch0 = 'z';
    let ch1 = '😻';

    println!("ch0: {}", ch0);
    println!("ch1: {}", ch1);
}

fn complex_types() {
    // tuple: conjunto de longitud determinada que representa un grupo de valores.
    // Se define con '()'. Los elementos se separan con comas.
    let tup: (&str, i32, bool) = ("str", 123, true);

    // Destructuración
    let (_s, _n, _b) = tup;
    
    // Notación de puntos (índices)
    let s = tup.0;
    let n = tup.1;
    let b = tup.2;
    // let error = tup[3];  // Índice no existe
    
    println!("s: {}", s);
    println!("n: {}", n);
    println!("b: {}", b);

    // Macro para imprimir toda la tupla
    println!("{:?}", tup);
    println!("{tup:?}");
    
    
    /* array: tamaño fijo. Se declara usando la sintaxis [T; N].
    * - T: tipo de dato que va a almacenar.
    * - N: tamaño de array.
    */
    let arr0: [i32; 3] = [200, 404, 500];
    
    // Para acceder a los datos se puede hacer usando [idx].
    let _not_found = arr0[1];
    // let error = arr0[3];  // Índice no existe
    
    // Inicialización del array
    let arr1 = [0; 4];
    
    // Macro para imprimir todo el array
    println!("{:?}", arr0);
    println!("{arr1:?}");
}

fn main() {
    concepts();
    simple_types();
    complex_types();
}
