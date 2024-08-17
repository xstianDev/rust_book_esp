//////////////////////////////
// 15.3. Drop Trait
// https://doc.rust-lang.org/stable/book/ch15-03-drop.html
//////////////////////////////

fn drop_trait() {
    /* Drop trait: permite personalizar qué pasa cuando un valor se sale de scope.
     * Por ejemplo, en el smart pointer Box, cuando sale del scope queremos que se 
     * desaloje la memoria del heap.
     */

    struct MyBox {
        data: String,
    }

    impl Drop for MyBox {
        fn drop(&mut self) {
            println!("Dropping MyBox with data `{}`!", self.data);
        }
    }

    let a = MyBox {
        data: String::from("my data"),
    };
    let b = MyBox {
        data: String::from("other data"),
    };

    println!("MyBoxes created");
    // Primero hará drop de "b" y luego de "a"
}

fn drop_trait_method() {
    struct MyBox {
        data: String,
    }

    impl Drop for MyBox {
        fn drop(&mut self) {
            println!("Dropping MyBox with data `{}`!", self.data);
        }
    }

    let c = MyBox {
        data: String::from("my data"),
    };

    /* La llamada explícita c.drop() no funciona ya que Rust llamará a su función destructora 
     * cuando acabe el scope, lo que podría provocar una doble eliminación de la memoria.
     * Por ello, se puede llamar a la función destructora de Rust.
     */
    println!("MyBox created");
    drop(c);
    println!("MyBox dropped");
}

fn main() {
    drop_trait();
    drop_trait_method();
}
