//////////////////////////////
// 18.1. Matching
// https://doc.rust-lang.org/stable/book/ch18-00-patterns.html
// https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html
// https://doc.rust-lang.org/stable/book/ch18-02-refutability.html
//////////////////////////////

fn patterns() {
    /* Los patrones son una sintaxis especial para hacer matching sobre estructuras.
     * 
     * Consisten en una combinación de:
     * - Literales
     * - Destructured (arrays, enums, structs, tuples)
     * - Variables
     * - Wildcards
     * - Placeholders
     */
}

fn matching_arms() {
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    // Las expresiones match son exhaustivas (todos los casos deben ser tratados)
    match language {
        Language::English => println!("English"),
        Language::Spanish => println!("Spanish"),
        Language::Russian => println!("Russian"),
        lang => println!("Unsupported language! {:?}", lang)
    }
}

fn matching_if_let() {
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    }
}

fn matching_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn matching_for() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn matching_let() {
    // let PATTERN = EXPRESSION;
    let (x, y, z) = (1, 2, 3);
}

fn matching_function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
    
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
}

fn matching_refutable_irrefutable() {
    // Irrefutable (siempre hacen match)
    // Solo aceptan patrones irrefutables: function parameters, let statements, for loops
    let x = 5;

    // Refutable (no siempre hacen match)
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }
}

fn main() {
    patterns();

    matching_arms();
    matching_if_let();
    matching_while_let();
    matching_for();
    matching_let();
    matching_function_parameters();
    matching_refutable_irrefutable();
}
