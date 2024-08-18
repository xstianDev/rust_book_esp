//////////////////////////////
// 19.1. Unsafe Rust
// https://doc.rust-lang.org/stable/book/ch19-00-advanced-features.html
// https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html
//////////////////////////////

use core::slice;

fn unsafe_code() {
    /* El código no seguro en Rust existe por:
     * - El análisis estático es conservador (Rust puede rechazar un programa si no puede garantizar
     *   que sea seguro, aunque el desarrollador sepa que sí lo es)
     * - El hardware es no seguro inherentemente (el sistema debe hacer tareas que no son memory safe)
     * 
     * Como Rust es un lenguaje de bajo nivel, debe permitir hacer operaciones no seguras.
     * Para ello se usa la palabra clave "unsafe". Esto ofrece nuevas capacidades:
     * - Dereferenciar raw pointers.
     * - Llamar funciones no seguras.
     * - Acceder o modificar una variable mutable estática. 
     * - Implementar un trait no seguro.
     * - Acceder campos de uniones.
     * 
     * Hacer código no seguro no desactiva el borrow checker ni las comprobaciones de seguridad.
     * El código no seguro debería usarse poco, y también se puede usar dentro de una abstracción de 
     * código seguro para dar una API segura.
     */
}

fn unsafe_dereference() {
    /* El código no seguro tiene dos tipos de raw pointers similares a las referencias:
     * - immutable raw pointer (no puede ser asignado después de ser dereferenciado)
     * - mutable raw pointer
     * 
     * El uso del carácter '*' no es una derefencencia, sino que indica que es un raw pointer.
     * 
     * smart pointers vs. raw pointers:
     * - Los smart pointers pueden ignorar las reglas de borrowing teniendo punteros mutables e inmutables
     *   o varios punteros mutables a la misma ubicación en memoria.
     * - Los raw pointers no garantizan apuntar a memoria válida.
     * - Los raw pointers pueden ser null.
     * - Los raw pointers no implementan una limpieza automática.
     * 
     * Rust permite crear raw pointers, pero no permite dereferenciarlos fuera de un bloque unsafe.
     * Los raw pointers son útiles al intercalar código de C o haciendo abstracciones seguras que
     * el borrow checker no entiende.
     */
    let mut num = 5;
    
    // immutable raw pointer
    let r1 = &num as *const i32;
    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    /* En este caso, no sabemos si la dirección de memoria tiene datos.
    let address = 0x012345usize;
    let r3 = address as *const i32;
    */

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn unsafe_functions() {
    // La función tiene requerimientos no seguros
    unsafe fn dangerous() {}
    
    // Las funciones no seguras se llaman dentro de bloques o funciones no seguros
    unsafe {
        dangerous();
    }
}

fn unsafe_static_mut_variable() {
    /* Rust soporta variables globales, pero pueden ser conflictivas con las reglas de ownership.
     * Pueden suponer una race condition al acceder al dato y por eso son inseguras.
     * 
     * Las variables globales son como constantes. Utilizan la palabra 'static', ya que ese es su lifetime.
     * También se debe anotar su tipo y no es necesario inferirle el lifetime ya que el compilador lo hace.
     * 
     * Las variables estáticas tienen un una dirección fija en memoria, por lo que siempre se accede 
     * al mismo dato. Las constantes pueden duplicar sus datos cuando se usan, por lo que el compilador
     * puede reemplazar las referencias por el dato directamente.
     */
    static HELLO_WORLD: &str = "Hello, world!";
    static mut COUNTER: u32 = 0;

    println!("{}", HELLO_WORLD);
    
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn unsafe_traits() {
    // Un trait es inseguro cuando al menos uno de sus métodos es inseguro.
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}

fn unsafe_unions() {
    /* Una unión es similar a un struct pero solo se usa un field para cada instancia.
     * Sobre todo se usan para comunicarse con los unions de C, y es inseguro acceder campos de una
     * unión ya que Rust no puede asegurar el tipo de dato que está accediendo.
     */
}

fn safe_abstraction() {
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        /* (&mut slice[..mid], &mut slice[mid..])
         *
         * Esta implementación no funciona, ya que accedemos dos veces a 'slice' y el borrow checker no
         * comprende la operación. Por ello hay que hacer un bloque no seguro. 
         */

        unsafe {(
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )}
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn extern_functions() {
    // Importación de una función de C usando su ABI (Application Binary Interface)
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // Llamar a una función externa es inseguro ya que no tenemos control sobre sus garantías de seguridad.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Se pueden exportar funciones de Rust para que las usen otros lenguajes
    // #[no_mangle]: no cambiar el nombre de la función cuando se compila a otro lenguaje.
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
}

fn main() {
    unsafe_code();

    unsafe_dereference();
    unsafe_functions();
    unsafe_static_mut_variable();
    unsafe_traits();
    unsafe_unions();

    safe_abstraction();
    extern_functions();
}
