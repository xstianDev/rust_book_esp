//////////////////////////////
// 19.3. Advanced Types
// https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html
//////////////////////////////

fn newtype_pattern() {
    /* Patrón newtype: según la orphan rule, se puede implementar un trait en un type mientras 
    * el trait o el type estén definidos dentro del crate.
    * 
    * El patrón newtype permite saltarse esta restricción creando un struct de tupla con un campo.
    * Por ejemplo, se puede usar para añadir el trait Display a un vector. 
    */
    
    use std::fmt;
    
    struct Wrapper(Vec<String>);
    
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    
    /* En caso de que un código reciba dos parámetros del mismo tipo, estos se pueden envolver
     * en un nuevo struct el cual encapsule mejor los valores.
     */
    struct Age(u32);
    struct ID(u32);

    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);
}

fn type_alias() {
    // Los alias le dan nuevos nombre a tipos existentes. Se usan para evitar repetición
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);


    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = 
        Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {}
}

fn never_type() {
    /* El tipo never se representa con '!'. Se usa cuando:
     * - Usamos "continue"
     * - Usamos panic!()
     * - Un loop (menos si usa break)
     */
    fn bar() -> ! {
        // --snip--
    }
}

fn dynamically_sized_types() {
    /* Los tipos de tamaño dinámico (DST) son aquellos cuyo tamaño solo se puede saber en ejecución.
     * Un ejemplo es el tipo "str", cuyo tamaño no se conoce en compilación.
     * 
     * Para poder usar el tipo "str", tenemos que usar la versión borrowed "&str", que guarda:
     * - La dirección de memoria apuntando al valor en memoria
     * - La longitud del string.
     * 
     * Ambos valores son de tipo "usize", lo que hace que el código funcione.
     * 
     * La regla para los DST es que siempre hay que ponerlos detrás de algún puntero, como la referencia
     * o un smart pointer.
     * 
     * Los traits también son DST. Rust tiene un trait llamado "Size" que determina si un tipo puede ser
     * conocido en compilación o no. Este trait es implementado en todo tipo cuyo tamaño se conocen en
     * tiempo de compilación y a las funciones genéricas.
     */

    let s1: &str = "Hello there!";
    let s2: &str = "How is it going?";

    // T puede ser Sized o no
    fn generic<T: ?Sized>(t: &T) {}
}

fn main() {
    newtype_pattern();
    type_alias();
    never_type();
    dynamically_sized_types();
}
