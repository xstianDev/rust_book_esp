//////////////////////////////
// 10.3. Lifetimes
// https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
//////////////////////////////

fn lifetimes_borrow_checker() {
    // El borrow checker comprueba en compilación que no haya referencias colgantes

    let r;

    {
        let x = 5;
        // r = &x;  // `x` does not live long enough; borrowed value does not live long enough
        
        // r no puede tomar la referencia de x más allá de este punto porque el lifetime
        // de x acaba en el mismo lugar donde termina el scope
    }

    println!("r: {r}");
}

fn lifetimes_generical_annotations() {
    /* Generical lifetime annotations: explican la relación entre referencias y sus lifetimes, sin cambiar este
     * Se usa ya que a veces el borrow checker no puede saber el lifetime de los parámetros
     * Los lifetimes se anotan con un ' + una letra (a -> z)
     */

    // &i32        // referencia
    // &'a i32     // referencia con lifetime explícito
    // &'a mut i32 // referencia mutable con lifetime explícito

    /* El lifetime del valor devuelto toma el lifetime más pequeño de los argumentos
     * Si el más pequeño es el de x, el lifetime del valor devuelto será ese (igual si fuese y)
     * Si solo devolviésemos x, podríamos quitar el 'a de y, ya que no nos importa su lifetime
     * El lifetime del valor devuelto debe depender de un parámetro pasado
     */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let str1 = String::from("abc");
    let str2 = String::from("xyz");
    
    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is {}", result);
}

fn lifetimes_structs() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn lifetimes_elision() {
    // input lifetime: el del parámetro pasado
    // output lifetime: el del valor devuelto

    /* Reglas del "lifetime elision":
     * 1. Cada parámetro es una referencia a su propio lifetime
     * 2. Si solo hay un input, el output comparte el mismo lifetime
     * 3. Si hay varios input, pero uno es &self o &mut self,
     *    el lifetime de self se asigna a todos los output
     */

    // No es necesario pasar los lifetimes aquí, ya que se infieren en compilación
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}

fn lifetimes_method_definitions() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // Los lifetimes son un tipo genérico
    impl<'a> ImportantExcerpt<'a> {
        // ('a &self, announcement: &str) -> &'a str {
        fn return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

fn lifetimes_static() {
    // 'static: tipo de lifetime especial que dura toda la ejecución del programa.

    // Los string literal tienen un lifetime estático, ya que se guardan en el binario del programa.  
    let s: &'static str = "I have a static lifetime.";
}

fn generics_traits_and_lifetimes() {
    use std::fmt::Display;

    fn longest_and_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() { x } else { y }
    }
}

fn main() {
    lifetimes_borrow_checker();
    lifetimes_generical_annotations();
    lifetimes_structs();
    lifetimes_elision();
    lifetimes_method_definitions();
    lifetimes_static();

    generics_traits_and_lifetimes();
}
