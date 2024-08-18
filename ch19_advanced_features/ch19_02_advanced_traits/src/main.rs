//////////////////////////////
// 19.2. Advanced Traits
// https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html
//////////////////////////////

fn associated_types() {
    /* Los associated types son placeholders que se añaden al trait, y sus métodos pueden usar usarlos.
     * 
     * La diferencia entre los tipos asociados y los genéricos es que los primeros solo permiten un tipo
     * concreto por implementación, mientras que los segundos permiten cualquier tipo.
     * 
     * Por tanto, los tipos asociados solo pueden tener una implementación, mientras que los genéricos
     * pueden tener más de una.
     */

    pub trait Iterator {
        type Item;

        // Devuelve el siguiente dato en un iterador, cuyo tipo no sabemos
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {}

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(0)
        }
    }
}

fn default_generic_type_parameters() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Sin tipo por defecto
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    struct Millimeters(u32);
    struct Meters(u32);

    // Tipo por defecto
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn methods_with_same_name() {
    trait Pilot {
        fn fly();
    }

    trait Wizard {
        fn fly();
    }

    struct Human;
    impl Human {
        fn fly(){
            println!("*waving arms furiously*");
        }
    }

    impl Pilot for Human {
        fn fly() {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly() {
            println!("Up!");
        }
    }

    // Sin referencia a self en la función
    Human::fly();
    <Human as Pilot>::fly();
    <Human as Wizard>::fly();
}

fn supertrait() {
    use std::fmt;

    // El supertrait es fmt::Display, ya que OutlinePrint depende de él
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

fn newtype_pattern() {
    /* Patrón newtype: según la orphan rule, se puede implementar un trait en un type mientras 
     * el trait o el type estén definidos dentro del crate.
     * 
     * El patrón newtype permite saltarse esta restricción creando un struct de tupla con un campo.
     * Por ejemplo, se puede usar para añadir el trait Display a un vector. 
     */

    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);
}
fn main() {
    associated_types();
    default_generic_type_parameters();
    methods_with_same_name();
    supertrait();
    newtype_pattern();
}
