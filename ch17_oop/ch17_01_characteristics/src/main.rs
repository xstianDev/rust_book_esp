//////////////////////////////
// 17.1. Characteristics
// https://doc.rust-lang.org/stable/book/ch17-00-oop.html
// https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html
//////////////////////////////

fn objects() {
    /* Objetos: estructuras que contienen datos y funciones que usan esos datos.
     *
     * En Rust, no existen los objetos como tal, pero los enums y structs contienen datos
     * y con los bloques impl podemos definir funciones.
     */
}

fn encapsulation() {
    // Encapsulación: los detalles de implementación de un objeto están ocultos al código.
    // En Rust, todo es privado hasta que se usa "pub" (público).
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }
    
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn inheritance() {
    /* En Rust, la herencia se hace con traits, los cuales pueden definir métodos pero no campos,
     * aunque eso último es una propuesta de la comunidad.
     */
}

fn main() {
    objects();
    encapsulation();
    inheritance();
}
