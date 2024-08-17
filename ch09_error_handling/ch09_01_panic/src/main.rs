//////////////////////////////
// 9.1. Panic
// https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html
// https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html
// https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html
// https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html
//////////////////////////////

fn unrecoverable() {
    // Los panic son los errores de Rust.

    // Los errores irrecuperables son los que rompen la ejecución
    fn a() {
        b(1);
    }

    fn b(num: i32) {
        if num == 1 {
            // Para mostrar la traza usamos RUST_BACKTRACE=1 cargo run
            panic!("Invalid value");
        }
    }

    a();
}

fn recoverable() {
    // Los errores recuperables son los que no rompen la ejecución
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt");

    // Result<T, E>
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        }
    };
    
    // Closures
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });


    // Expect
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagation() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file_with_operator_and_chaining() -> Result<String, io::Error> {
        /* El operador ? omite el Result<T, E>:
         * Ok: devuelve el valor.
         * Err: devuelve el error
         */
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
}

fn panic_vs_result() {
    /* Cuando usar panic! o Result: 
     * En general, debería usase Result y la propagación de errores. Evita que el programa se bloquee y permite manejar el error.
     * Se usa panic! (además de expect y unwrap):
     * - Cuando no es posible recuperarse del error.
     * - Código de ejemplo, donde no hay contexto para manejar los errores.
     * - Código protitipo, donde no quieres manejar los errores.
     * - Código de test, donde quieres que el test falle
     * - Cuando sabes que una llamada a una función tendrá éxito.
     */

    use std::net::IpAddr;
    
    // Al ser un valor puesto a mano, siempre va a dar Ok 
    let home: IpAddr = "127.0.0.1".parse().unwrap(); 
}

fn validation() {
    // Podemos crear tipos personalizados para la validación

    pub struct Guess {
        value: i32
    }

    impl Guess {
        pub fn new(value: i32) -> Guess { 
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

fn main() {
    unrecoverable();
    recoverable();
    propagation();
    panic_vs_result();
    validation();
}