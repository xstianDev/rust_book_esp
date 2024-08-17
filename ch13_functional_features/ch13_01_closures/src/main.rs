//////////////////////////////
// 13.1. Closures
// https://doc.rust-lang.org/stable/book/ch13-00-functional-features.html
// https://doc.rust-lang.org/stable/book/ch13-01-closures.html
//////////////////////////////

use std::thread;
use std::time::Duration;

fn closures() {
    /* Closures: son como funciones anónimas (sin nombre) que se pueden almacenar en variables
     * Como su caso de uso es más limitado, no es necesario tiparlos (el compilador puede hacerlo)
     * En vez de usar (param) usa |param|
     * Los closures usan uno de los tres traits de función que hay: Fn, FnMut o FnOnce
     */
    fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn generate_workout_without_closures(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                "Today, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }

    // Para tipar items que usan closures, necesitamos generics y trait bounds
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout_with_closures(intensity: u32, random_number: u32) {
        // Closure (guarda la función, no el resultado)
        let mut cached_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!(
                "Today, do {} pushups!",
                cached_result.value(intensity)
            );
            println!(
                "Today, do {} situps!",
                cached_result.value(intensity)
            );
        } else {
            if random_number == 3 {
                println!("Take a break today!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    cached_result.value(intensity)
                );
            }
        }
    }

    let simulated_intensity = 10;
    let simulated_random_number = 7;
}

fn closures_vs_fn() {
    /* Los closures ocupan más memoria ya que guardan el contexto de su scope
     * Toman los valores de su entorno de las mismas tres maneras que las funciones:
     * - Tomando ownership (FnOnce) (solo se puede llamar una vez al closure)
     * - Borrowing mutable (FnMut)
     * - Borrowing inmutable (Fn)
     * 
     * Rust infiere estos traits al closure según usamos los valores del entorno
     */

    let x = 4;
    let y = 4;

    /*
    fn equal_to_x (z: u32) -> bool {
        z == x  // can't capture dynamic environment in a fn item; use the `|| { ... }` closure form instead
    }
    */
    
    // Puede acceder a 'x' porque comparten scope, pero una función no puede
    let equal_to_x = |z| z == x;
    assert!(equal_to_x(y));
}

fn closures_move() {
    // Podemos forzar al closure a tomar ownership de los valores usados en su entorno con 'move'
    // Útil cuando pasamos un closure de un hilo a otro, pasando también el ownership
    
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 3];
    
    // Puede acceder a 'x' porque comparten scope, pero una función no puede
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);  // borrow of moved value: `x`; value borrowed here after move
    assert!(equal_to_x(y));
}

fn main() {
    closures();
    closures_vs_fn();
    closures_move();
}
