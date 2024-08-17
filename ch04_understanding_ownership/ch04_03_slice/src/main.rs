//////////////////////////////
// 4.3. Slice
// https://doc.rust-lang.org/book/ch04-03-slices.html
//////////////////////////////

// Slices: referencia una secuencia de elementos dentro de una colección.

/* Problemas de la función:
 * - El resultado depende de la ejecución. Después de s.clear(), el valor de word sigue siendo 5. 
 * - Solo obtiene la primera palabra
 */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return i;
        }
    }

    s.len()
}

fn first_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word: {}", word);
    
    s.clear();
    println!("clear: {}", word);
    
    println!("s1: {}", &s[..5]);    // Equivalente a &s[0..5]
    println!("s2: {}", &s[6..]);    // Equivalente a &s[6..11]
    
    // No da error porque no llega el scope de las referencias inmutables
    let s1 = "hello world";
    let slice = first_slice(&s1);
    println!("slice: {}", slice);
    
    // Referencia a un array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[..2];
    println!("slice: {:?}", &slice[..]);
}
