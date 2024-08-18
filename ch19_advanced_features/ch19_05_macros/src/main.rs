//////////////////////////////
// 19.5. Macros
// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
//////////////////////////////

fn macros() {
    /* Los macros permiten escribir código que escribe otro código (metaprogramación).
     * Son como funciones que reciben y devuelven código. Permiten reducir la cantidad de código,
     * como las funciones, aunque tienen diferencias.
     * 
     * funciones vs. macros:
     * - Las funciones deben declarar el número de parámetros que aceptan, mientras que los macros pueden 
     *   aceptar un número variable de de parámetros.
     * - Las funciones se llaman en ejecución, mientras que las macros se expanden antes de acabar la compilación. 
     * - Las macros añaden complejidad al código.
     * 
     * Existen dos tipos de macros:
     * - Declarativas
     * - Procedurales
     */
}

fn macros_declarative() {
    /* Las macros declarativas son las más comunes. Permiten escribir algo similar a un "match".
     * Debido a los edge cases que puede generar macro_rules, está planeado deprecarlo en el futuro
     * a favor de otra alternativa.
     */ 

    let v1: Vec<u32> = vec![1, 2, 3];
    let v1: Vec<&str> = vec!["a", "b", "c", "d", "e"];
}

fn macros_procedural() {
    /* Las macros procedurales son como funciones que toman código, operan con él y devuelven código.
     *
     * Hay 3 tipos de macros procedurales:
     * - Custom derived
     * - Attribute-like
     * - Function-like
     * 
     * Por razones técnicas complejas que el equipo de Rust desea eliminar, los macros procedurales deben
     * estar definidos en su propio crate con un tipo personalizado de crate.
     */
}

fn macros_procedural_custom_derived() {
    // Hay que importar estas macros como dependencias en Cargo.toml
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    Pancakes::hello_macro();
}

fn macros_procedural_attribute_like() {
    /* Las macros attribute-like son parecidas a las custom derived, excepto que en vez de generar
     * código para el atributo derive, pueden crear un atributo customizado.
     * 
     * Además, las custom derived macros solo funcionan en structs y enums, mientras que las 
     * attribute-like pueden funcionar en otros tipos (por ejemplo, funciones).
     */

    struct TokenStream {}

    #[route(GET, "/")]
    fn index() {}

    #[proc_macro_attribute]
    pub fn route(
        attr: TokenStream,
        item: TokenStream,
    ) -> TokenStream {}
}

fn macros_procedural_function_like() {
    /* Las function-like macros parecen llamadas a funciones, pero son más flexibles.
     * Pueden tomar un número variable de argumentos y operan con código de Rust.
     */

    let sql = sql!(SELECT * FROM posts WHERE id=1);

    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {}
}


fn main() {
    macros();
    macros_declarative();
    macros_procedural();
    macros_procedural_custom_derived();
    macros_procedural_attribute_like();
    macros_procedural_function_like();
}
