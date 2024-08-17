//////////////////////////////
// 8.2. String
// https://doc.rust-lang.org/stable/book/ch08-02-strings.html
//////////////////////////////

use unicode_segmentation::UnicodeSegmentation;

fn strings() {
    // Los string se guardan como una colección de caracteres UTF-8

    // String: tipo dinámico que se guarda en el heap.
    // &str: secuencia inmutable de caracteres UTF-8 de longitud dinámica que se guarda en memoria.
    let s1 = String::new();
    let s2 = "text";
    let s3 = s2.to_string();
    let s4 = String::from("text");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
}

fn concatentation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    
    // Concatenación: ownership de s1 -> s3 + texto de s2
    // let s3 = s1 + &s2;

    // format! no toma ownership e interpola los strings
    let s3 = format!("{}{}", s1, s2);
}

fn characters() {
    // Los caracteres de un String no se pueden indexar, ya que un caracter puede ocupar más de 1 byte.

    let hello: String = String::from("Hello");
    // let c: char = hello[0];     // the type `String` cannot be indexed by `{integer}`

    let namaste = String::from("नमस्ते");
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Scalar values (char)
    // ['न', 'म', 'स', '्', 'त', 'े']

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Grapheme clusters
    // ["न", "म", "स्", "ते"]
    
    // Por defecto no existen los graphemes en Rust, así que se descarga "unicode_segmentation"
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
}

fn main() {
    strings();
    concatentation();
    characters();
}
