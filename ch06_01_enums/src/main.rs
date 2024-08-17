//////////////////////////////
// 6.1. Enums
// https://doc.rust-lang.org/stable/book/ch06-00-enums.html
// https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html
//////////////////////////////

fn enums() {
    // Enum: lista de variantes. Permite datos en las 'variants'. También pueden usar 'impl'
    enum IpAddrKind {
        V4(u8, u8, u8, u8),     // variant con datos 
        V6,                     // variant sin datos
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let mac = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("00-B0-D0-63-C2-26")
    };

    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn enum_vs_struct() {
    enum Message {
        Quit,                                   // no data
        Move { x: i32, y: i32 },                // anonymous struct
        Write(String),                          // String
        ChangeColor(i32, i32, i32),             // 3 ints
    }

    /*
    Equivalente enum 'Message'
    Los struct tienen tipos diferentes. En el enum comparten el tipo 'Message'
     */
    struct QuitMessage;                         // unit struct
    struct MoveMessage {                        // struct
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);                // tuple struct
    struct ChangeColorMessage(i32, i32, i32);   // tuple struct    
}

fn option_enum() {
    /* Los lenguajes de programación suelen tener un tipo null.
     * El sistema de tipos no puede garantizar que el valor usado no sea null.
     * Rust no tiene null, sino que usa un enum Option<T> incluido en el scope del programa.
    
    enum Option<T> {
        Some(T),
        None,
    }
     */
    
    let some_num = Some(5);
    let some_str = Some("string");

    let null_num: Option<i32> = None;   // Hace falta anotar el tipo para que no sea Option<{unknown}>

    // No se permiten operaciones entre un valor y un opcional
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;

    // let sum = x + y;    
    // cannot add `Option<i8>` to `i8`; the trait `Add<Option<i8>>` is not implemented for `i8`
    
    // Para operar un valor y un opcional hay que extraer el valor de este
    let sum_xy = x + y.unwrap_or(0);    // 10
    let sum_xz = x + z.unwrap_or(0);    // 5
}

fn main() {
    enums();
    enum_vs_struct();
    option_enum();
}
