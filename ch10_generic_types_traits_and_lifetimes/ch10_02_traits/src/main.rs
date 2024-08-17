//////////////////////////////
// 10.2. Generics
// https://doc.rust-lang.org/stable/book/ch10-02-traits.html
//////////////////////////////

fn trait_impl() {
    // Un trait define lo que debe tener cada tipo que lo implemente
    pub trait Summary {
        // Implementación como clase abstracta de otros lenguajes        
        fn summarize_author(&self) -> String;
        
        // Implementación por defecto
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    // Implementación del trait
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }

        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    // Implementación del trait
    impl Summary for Tweet {
        // Usa la implementación por defecto de "summarize"
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}

fn trait_bounds() {
    // trait bound: cuando el parámetro implementa el trait
    // El syntactic sugar de un trait bound usa "impl" en el parámetro
    // Para mejorar la legibilidad podemos usar las "where clauses" 

    use std::fmt::{Debug, Display};

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Un trait implementado
    fn trait_bound_1(item: &impl Summary) {} 
    fn trait_bound_2<T: Summary>(item: &T) {} 

    // Más de un trait implementado
    fn trait_bound_3(item1: &(impl Summary + Display), item2: &impl Summary) {}
    fn trait_bound_4<T: Summary + Display>(item1: &T, item2: &T) {}

    // where clause
    fn trait_bound_5<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
    fn trait_bound_6<T, U>(t: &T, u: &U)
        where T: Display + Clone,
              U: Clone + Debug
    {}
}

fn trait_return() {
    // Se pueden crear funciones que devuelven un item que implemente el trait

    pub trait Summary {
        fn summarize(&self) -> String {
            format!("(Read more...)")
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {}

    // La función solo puede devolver un tipo. No podemos hacer un if con Tweet y NewsArticle
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse-ebooks"),
            content: String::from("New book!"),
            reply: false,
            retweet: false,
        }
    }

    println!("{}", returns_summarizable().summarize());
}

fn trait_conditional_impl() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Solo para los tipos que implementen Display + PartialOrd  
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn trait_blanket() {
    // Podemos implementar un trait en un tipo que implementa otro trait

    // Blanket implementation
    /*
    impl<T: Display> ToString for T {
        // --snip--
    }
    */
}

fn main() {
    trait_impl();
    trait_bounds();
    trait_return();
    trait_conditional_impl();
    trait_blanket();
}
