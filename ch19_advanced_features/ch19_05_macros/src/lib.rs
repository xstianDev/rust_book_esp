/* El match que hacen las macros es diferente que los match normales, ya que hacen match sobre código en
 * vez de sobre valores.
 * 
 * Sintaxis de la macro vec! :
 * - #[macro_export]: cuando la crate en la que se define la macro entra al scope, permite usar la macro.
 * - macro_rules! [name]: define la macro.
 * 
 * - ( $( $x:expr ),* ): patrón.
 *   - (...): encapsula todo.
 *   - $(...): captura todos los valores que coinciden con el patrón contenido.
 *     - $x:expr : el patrón captura todo el código que coincide con cualquier expresión y lo asigna a $x.
 *       Es el código que se usa para el reemplazo.
 *   - ,: puede aparecer una coma después del patrón anterior.
 *   - *: el patrón puede coincidir 0 o más veces.
 * 
 * - $(...)* : genera la línea contenida para cada vez que coincide el patrón.
 */

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            /* Equivale a:
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            ...
             */
            temp_vec
        }
    };
}
