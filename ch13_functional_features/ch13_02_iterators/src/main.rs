//////////////////////////////
// 13.2. Iterators
// https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
// https://doc.rust-lang.org/stable/book/ch13-04-performance.html
//////////////////////////////

fn iterators() {
    // Iterador: permite iterar sobre una secuencia de elementos sin importar como están almacenados
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // iterator consumer
    for value in v1_iter {
        println!("Got: {}", value);
    }
}

fn iterators_adaptor_method() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let expected = [2, 3, 4];
    assert_eq!(v2, expected);
}

fn main() {
    iterators();
    iterators_adaptor_method();
}
