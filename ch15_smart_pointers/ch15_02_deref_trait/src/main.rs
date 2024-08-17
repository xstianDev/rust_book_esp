//////////////////////////////
// 15.2. Defer Trait
// https://doc.rust-lang.org/stable/book/ch15-02-deref.html
//////////////////////////////

fn deref_trait() {
    /* Deref trait: permite cambiar el comportamiento del operador de desreferencia (*)
     * En el ejemplo, "y" guarda la referencia de "x" (&), y en el assert_eq! 
     * sigue esa referencia para obtener el valor con el operador de desreferencia (*)
     * 
     * También podemos usar Box en vez de &, ya que implementa el deref trait y permite al
     * deref operator a trabajar igual que si Box fuese una referencia.
     */

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

fn deref_trait_smart_pointer() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl <T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);              // Deref trait
    assert_eq!(5, *(y.deref()));    // Internamente (equivale a la línea superior)
}

fn implicit_deref_coercion() {
    /* Se realiza el deref coercion en tres casos:
     * - Paso de &T a &U cuando T: Deref<Target=U>
     * - Paso de &mut T a &U cuando T: Deref<Target=U>
     * - Paso de &mut T a &mut U cuando T: DerefMut<Target=U>
     */
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl <T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);              // Internamente: &MyBox<String> -> &String -> &str
    hello(&(*m)[..]);       // Equivalente
}

fn main() {
    deref_trait();
    deref_trait_smart_pointer();
    implicit_deref_coercion();
}
