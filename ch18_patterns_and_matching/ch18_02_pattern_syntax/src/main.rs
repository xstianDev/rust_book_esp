//////////////////////////////
// 18.2. Pattern Syntax
// https://doc.rust-lang.org/stable/book/ch18-03-pattern-syntax.html
//////////////////////////////

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
}

fn matching_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }
    
    let x = 'c';
    
    match x {
        'a'..='j' => println!("a through j"),
        'k'..='z' => println!("k through z"),
        _ => println!("anything"),
    }
}

fn matching_destructuring_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x)
        },
        Point { x: 0, y } => {
            println!("On the y axis at {}", y)
        },
        Point { x, y } => {
            println!("On neither axis: ({}, {})", x, y)
        },
    }
}

fn matching_destructuring_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(
        Color::Hsv(0, 160, 255)
    );

    match msg {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y } => {
            println!(
                "Move to x: {}, y: {}",
                x, y
            );
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color: red {}, green {}, blue {}",
                r, g, b
            )
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color: hue {}, saturation {}, value {}",
                h, s, v
            )
        },
        _ => (),
    }
}

fn matching_multiple_destructuring() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) =
        ((3, 10), Point { x: 3, y: -10 });
}

fn matching_ignore_values() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(2, 3);
}

fn matching_ignore_multiple_values() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn matching_ignore_some_values() {
    let nums = (2, 4, 8, 16, 32);

    match nums {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    } 
}

fn matching_ignore_variables() {
    let _x = 5; // ignore
    let y = 10;

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {     // borrow of partially moved value: `s`
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn matching_ignore_range() {
    let nums = (2, 4, 8, 16, 32);

    match nums {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    } 
}

fn matching_guards() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn matching_guards_no_shadowing() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
}

fn matching_multiple_guards() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn matching_at_operator() {
    /* At (@) operator: permite crear una variable con un valor a la vez que lo testeamos
     * para ver si coincide con un patrón. 
     */

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn main() {
    matching_literals();
    matching_named_variables();
    matching_multiple_patterns();
    matching_range();
    
    matching_destructuring_struct();
    matching_destructuring_enum();
    matching_multiple_destructuring();
    
    matching_ignore_values();
    matching_ignore_multiple_values();
    matching_ignore_some_values();
    matching_ignore_variables();
    matching_ignore_range();

    matching_guards();
    matching_guards_no_shadowing();
    matching_multiple_guards();

    matching_at_operator();
}
