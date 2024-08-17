//////////////////////////////
// 3.3. Control de flujo
// https://doc.rust-lang.org/book/ch03-05-control-flow.html
//////////////////////////////

use rand::Rng;

fn main() {
    let n = rand::thread_rng().gen_range(1, 20);

    // if - else if - else
    if n == 10 {
        println!("{} es igual a 10", n);
    } else if n > 10 {
        println!("{} es mayor que 10", n);
    } else {
        println!("{} es menor que 10", n);
    }

    // Operador ternario
    let a = if n > 10 { n } else { 0 };
    println!("a: {}", a);

    // loop
    let mut i = 0;
    loop {
        println!("idx: {}", i);
        i += 1;
        
        // No se le pueden quitar los {} cuando es una línea
        if i == 5 {
            break;
        }
    }

    // while
    let mut i = 5;
    while i != 0 {
        println!("idx: {}", i);
        i -= 1;
    }

    // for ... in
    let arr = [10, 20, 30, 40, 50];
    for elem in arr.iter() {
        println!("Elemento: {}", elem)
    }
    println!("Array: {:?}", arr.iter());  // Iter([10, 20, 30, 40, 50])
    
    // for con expresión iteradora (rango)
    for n in 1..4 {
        println!("Num: {}", n);
    }
}
