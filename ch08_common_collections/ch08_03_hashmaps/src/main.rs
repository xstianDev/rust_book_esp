//////////////////////////////
// 8.3. Hashmaps
// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html
//////////////////////////////

use std::collections::HashMap;

fn hashmaps() {
    /* Hashmap: guarda pares key value, donde ambos pueden ser de cualquier valor
     * Usa una función de hashing para saber donde coloca los pares en memoria.
     */

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // La key toma ownership
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hashmap_insert() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
}

fn hashmap_update() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    hashmaps();
    hashmap_insert();
    hashmap_update();
}
