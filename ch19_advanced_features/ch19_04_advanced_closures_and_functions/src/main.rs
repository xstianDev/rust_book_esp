//////////////////////////////
// 19.4. Advanced Closures and Functions
// https://doc.rust-lang.org/stable/book/ch19-05-advanced-functions-and-closures.html
//////////////////////////////

fn function_pointers() {
    /* Los punteros de funciones se usan con la palabra reservada "fn", igual que para declarar funciones.
     * También se pueden pasar closures (Fn, FnMut, FnOnce).
     * 
     * En caso de usar una función externa, como las de C, solo podremos usar punteros de funciones
     * ya que C no soporta closures.
     */

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    // f toma el puntero a una función
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn do_twice_trait_bound<T>(f: T, arg: i32) -> i32
    where T: Fn(i32) -> i32 {
        f(arg) + f(arg)
    }
    
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");


    let numbers: Vec<i32> = vec![1, 2, 3];
    let strings: Vec<String> = 
        numbers
        .iter()
        // .map(|i| i.to_string())
        .map(ToString::to_string)
        .collect();

    println!("{:?}", strings);


    enum Status {
        Value(u32),
        Stop,
    }

    let statuses: Vec<Status> =
        (0u32..20).map(Status::Value).collect();
}

fn returning_closures() {
    fn returns_closure_trait(a: i32) -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    fn returns_closure_dyn(a: i32) -> Box<dyn Fn(i32) -> i32> {
        // Cada closure representa un tipo diferente, aunque sean idénticos
        if a > 0 {
            Box::new(move |b| a + b)
        } else {
            Box::new(move |b| a - b)
        }
    }
}

fn main() {
    function_pointers();
    returning_closures();
}
