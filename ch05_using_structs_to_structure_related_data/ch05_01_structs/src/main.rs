//////////////////////////////
// 5. Structs
// https://doc.rust-lang.org/stable/book/ch05-00-structs.html
// https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html
// https://doc.rust-lang.org/stable/book/ch05-02-example-structs.html
// https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html
//////////////////////////////

fn structs0() {
    // Structs: agrupan datos relacionados. Se organizan en 'fields'.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool 
    }
    
    // Tuple struct: struct cuyos fields no tienen nombre.
    struct Point(i32, i32, i32);

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,   // Normal assign
            username,       // Field init shorthand syntax
            active: true,
            sign_in_count: 1,
        }
    }

    // No importa el orden de los fields
    let mut user1 = User {
        email: String::from("example@mail.com"),
        username: String::from("pepe123"),
        active: true,
        sign_in_count: 1,
    };

    // Manipulación de fields
    let name = user1.username;
    user1.username = String::from("pepillo");

    // Función generadora
    let user2 = build_user(
        String::from("pepelu@mail.com"), 
         String::from("pepelu")
    );

    // Relleno de campos por defecto
    let user3 = User {
        email: String::from("luis@mail.com"), 
        username: String::from("luis"),
        ..user2
    };
}

fn structs1() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }
    
    // Implementation block
    impl Rectangle {
        // Métodos: reciben self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        
        // Funciones asociativas: no necesitan una instancia del struct ni reciben self
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size
            }
        }
    }
    
    fn build_rect(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    let rect0 = build_rect(30, 50);
    let rect1 = build_rect(20, 40);
    let rect2 = build_rect(40, 50);
    let rect3 = Rectangle::square(20);

    println!("rect0 can hold rect1: {:?}", rect0.can_hold(&rect1));
    println!("rect0 can hold rect2: {:?}", rect0.can_hold(&rect2));
    println!("rect0 can hold rect3: {:?}", rect0.can_hold(&rect3));

    println!("area: {}px", rect0.area());

    /* Necesita el 'trait' std::fmt::Display para imprimir por consola (no vale {}).
     * Se añade (derived trait) a mano.
     * El error pide poner {:?} o {:#?}
     * 
     * Después de poner {:?} pide el trait Debug
     * Para ello se pone #[derive(Debug)] encima del struct
     */
    println!("rect: {:?}", rect0);
}

fn main() {
    structs0();
    structs1();
}