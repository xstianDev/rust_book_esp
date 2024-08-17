//////////////////////////////
// 6.2. Match
// https://doc.rust-lang.org/stable/book/ch06-02-match.html
//////////////////////////////

fn matching() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        // ...
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        // match es exhaustivo. Necesita tener cada posibilidad ('arm') controlada
        match coin {
            Coin::Penny => {
                // acción antes de devolver
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::California));
}

fn optional_matching() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            _ => None,      // En caso de tener muchas arms, permite '_' como el 'default' de otros lenguajes
        }
    }

    println!("Some: {:?}", plus_one(Some(5)));
    println!("None: {:?}", plus_one(None));
}

fn if_let() {
    let value = Some(3);
    match value {
        Some(3) => println!("Three!"),
        _ => ()
    }

    // 'if let' equivale a un solo arm
    if let Some(3) = value {
        println!("Three!")
    }
}

fn main() {
    matching();
    optional_matching();
    if_let();
}
