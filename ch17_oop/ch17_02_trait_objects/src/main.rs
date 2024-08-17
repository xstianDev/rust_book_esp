//////////////////////////////
// 17.2. Trait Objects
// https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html
//////////////////////////////

use ch17_02_trait_objects::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// static dispatch
impl Draw for SelectBox {
    fn draw(&self) {
        // draw button
    }
}

fn gui_lib() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok"),
            }),
        ]
    };

    screen.run();
}

fn dispatches() {
    /* Static dispatch: el compilador sabe las funciones concretas que llamamos
     * en tiempo de compilación.
     * 
     * Por ejemplo, cuando sucede la monomorfización (usamos generics y traits), es decir,
     * cuando el compilador genera código sustituyendo el tipo genérico por el tipo real.
     * 
     * 
     * Dynamic dispatch: el compilador no sabe las funciones concretas que llamamos
     * en tiempo de compilación, por lo que hace este proceso en ejecución.
     * 
     * Cuando usamos trait objects, debemos usar dynamic dispatch, ya que el compilador
     * no sabe todo lo que vamos a usar en tiempo de compilación. Esto causa un coste en
     * rendimiento a cambio de poder hacer un código flexible que acepta cualquier objeto
     * que implementa un trait concreto.  
     */
}

fn object_safety() {
    /* Solo se pueden hacer object safe traits dentro de los trait bounds.
     * Cuando en todos los método implementados en el trait:
     * - El return type no es self
     * - No hay parámetros genéricos
     * 
     * Si un trait no cumple estas dos propiedades, el compilador no puede
     * saber el tipo concreto del trait y no sabe a qué método llamar.
     */
}

fn main() {
    gui_lib();
    dispatches();
    object_safety();
}
