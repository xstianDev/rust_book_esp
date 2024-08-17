//////////////////////////////
// Ch 3. Common Programming Concepts
// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
//////////////////////////////

//////////////////////////////
// 3. Conceptos comunes de Rust
//////////////////////////////

fn main() {
    // CONCEPTOS DE RUST

    // las variables son inmutables por defecto.
    let x = 5;
    println!("El valor de x es {}", x);
    // x = 6;  // Error: la variable es inmutable.
    
    // Esta variable hace shadowing a la segunda.
    let x = "six";
    println!("El valor de x es {}", x);

    /* Características del shadowing:
     * - Se conserva la mutabilidad.
     * - Permite cambiar el tipo.
     */

    const _NUM: u32 = 100000;
    // const mut NUM: u32 = 100000;  // Error: las constantes no pueden ser mutables.

    /* Diferencias entre variables inmutables y constantes:
     * - Las inmutables no tienen por qué especificar su tipo, mientras que las constantes sí.
     * - Las inmutables pueden recibir cualquier valor, pero las constantes solo pueden tener un valor ya definido.
     *   Por ejemplo, una inmutable puede recibir un valor devuelto por una función, pero las constantes no pueden
     *   recibir un valor calculado en tiempo de ejecución.
     */
    //


    // TIPOS DE DATO EN RUST
    
    // Integers
    /* Según su tamaño, pueden ser de 8, 16, 32 (por defecto), 64 o 128 bit y 
     * tener signo (ej.: i8) o no tener signo (ej.: u8).
     * 
     * Hay un tipo especial "arch" que depende en la arquitectura, 
     * normalmente de 32 o 64 bits. Puede ser "isize" o "usize".
     */

    // Los números grandes pueden llevar _ para distinguir mejor los dígitos.
    let a = 98_123;         // Decimal
    let b = 0xff;           // Hex
    let c = 0o77;           // Octal
    let d = 0b1111_0000;    // Binario
    let e = b'A';           // Byte (solo u8)

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);


    /* En caso de sobrepasar el valor máximo, Rust entra en pánico en debug, mientras que 
     * en lanzamiento realizará "two's complement wrapping", es decir, que números que 
     * sobrepasan el valor máximo volverán al número mínimo (ej.: 256 -> 0, 257 -> 1...).
     */
    let f: u8 = 255;
    println!("{}", f);
    // println!("{}", f + 1);  // Pánico porque es debug


    // Floating-point numbers
    // Todos los números float tienen signo. Pueden ser f32 o f64 (por defecto).
    let g: f32 = 2.0;
    let h: f64 = 3.0;
    let i = 4.0;

    println!("{}", g);
    println!("{}", h);
    println!("{}", i);

    println!("{}", 5 + 10);
    println!("{}", 95.5 - 4.3);
    println!("{}", 4 * 30);
    println!("{}", 56.7 / 32.2);
    println!("{}", 43 % 5);


    // Booleans
    let t = true;
    let f = false;

    println!("{}", t);
    println!("{}", f);


    // Character
    // Representa un caracter Unicode entre comillas simples.
    let ch0 = 'z';
    let ch1 = '😻';

    println!("{}", ch0);
    println!("{}", ch1);


    // TIPOS COMPUESTOS

    // Tupla: conjunto de longitud determinada que representa un grupo de valores.
    // Se define con paréntesis y se separan los elementos con comas.
    let tup = ("str", 123, true);

    /* Para conseguir datos de las tuplas se puede hacer de dos maneras:
     * - Destructuración
     * - Notación de puntos (índices)
     */
    let (_s, _n, _b) = tup;
    
    let s = tup.0;
    let n = tup.1;
    let b = tup.2;
    // let error = tup[3];  // Índice no existe
    
    println!("{}", s);
    println!("{}", n);
    println!("{}", b);

    // Macro para imprimir toda la tupla
    println!("{:?}", tup);
    
    
    /* Array: su tamaño es fijo y se declara usando la sintaxis [T; N].
    * - T: tipo de dato que va a almacenar.
    * - N: tamaño de array.
    */
    let arr0 = [200, 404, 500];
    
    // Para acceder a los datos se puede hacer usando [idx].
    let _not_found = arr0[1];
    // let error = arr0[3];  // Índice no existe
    
    // Inicialización del array.
    let arr1 = [0; 4];
    
    
    // Macro para imprimir todo el array
    println!("{:?}", arr0);
    println!("{:?}", arr1);
    
}
