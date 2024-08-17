//////////////////////////////
// 10.1. Generics
// https://doc.rust-lang.org/stable/book/ch10-00-generics.html
// https://doc.rust-lang.org/stable/book/ch10-01-syntax.html
//////////////////////////////

fn generic_fn() {
    // Los generics, traits y lifetimes previenen la duplicación del código
    fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
        let mut largest = list[0];
        
        for current in list {
            if current > largest {
                largest = current;
            }
        }
        
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest i32 is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest char is {largest}");
}

fn generic_struct() {
    // Los tipos T y U pueden ser iguales o diferentes
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.0 };
}

fn generic_impl() {
    // T y U son lo mismo pero con distinto nombre
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<U> Point<U> {
        fn x(&self) -> &U {
            &self.x
        }        
    }

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5., y: 10.0 };
}

fn generic_mixup() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // Queremos que V y W puedan ser de diferentes tipos que T y U 
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point{
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn generic_performance() {
    /* Sobre impacto en el rendimiento de los tipos genéricos, el programa no será más lento por su uso.
     * En compilación, Rust hace una monomorfización en la que el código que usa genéricos se adapta
     * a cada tipo.
     */

    // Código real
    let integer = Some(5);
    let float = Some(5.0);


    // Monomorfización
    enum Option_i32 {
        Some(i32),
        None,
    }
    
    enum Option_f64 {
        Some(f64),
        None,
    }
    
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

fn main() {
    generic_fn();
    generic_struct();
    generic_impl();
    generic_mixup();
    generic_performance();
}