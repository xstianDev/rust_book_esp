//////////////////////////////
// 5. Control de flujo
//////////////////////////////

use rand::Rng;

fn main() {
    
    let n = rand::thread_rng().gen_range(1, 20);

    // if - else if - else
    if n == 10  {
        println!("{} es igual a 10", n);
    } else if n > 10 {
        println!("{} es mayor que 10", n);
    } else {
        println!("{} es menor que 10", n);
    }

    // Operador ternario
    let a = if n > 10 { n } else { 0 };
    println!("{}", a);

    // loop
    let mut i = 0;
    loop {
        println!("{}", i);
        i += 1;
        
        // No se le pueden quitar los {} cuando es una línea
        if i == 5 {
            break;
        }
    }

    // while
    let mut i = 5;
    while i != 0 {
        println!("{}", i);
        i -= 1;
    }

    // for ... in
    let arr = [10, 20, 30, 40, 50];
    // iter(&self) -> Iter<'_, T>: devuelve un objeto iterable
    // println!("{:?}", arr.iter());  // Iter([10, 20, 30, 40, 50])
    for elem in arr.iter() {
        println!("Elemento: {}", elem)
    }

    // for con expresión iteradora (rango)
    for n in 1..4 {
        println!("Num: {}", n);
    }

}
