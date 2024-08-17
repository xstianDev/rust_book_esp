//////////////////////////////
// 11.1. Tests
// https://doc.rust-lang.org/stable/book/ch11-00-testing.html
// https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
// https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html
// https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html
//////////////////////////////


#[derive(Debug)]
pub struct Rectangle {
    pub w: u32,
    pub h: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}


// Configuración del módulo para tests
// Tests unitarios
#[cfg(test)]
mod tests {
    use super::*;

    // assert_eq macro
    #[test]
    fn test_sum_is_correct() {
        assert_eq!(sum(2, 2), 4);
    }


    // assert_ne macro
    #[test]
    fn test_sum_is_incorrect() {
        assert_ne!(sum(2, 2), 5);
    }
    

    // panic macro
    #[test]
    fn failing_test() {
        panic!("Make this test fail");
    }


    // assert macro
    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle { w: 1, h: 1 };
        let r2 = Rectangle { w: 2, h: 2 };
    
        assert!(r2.can_hold(&r1));
    }
    
    #[test]
    fn larger_cannot_hold_smaller() {
        let r1 = Rectangle { w: 1, h: 1 };
        let r2 = Rectangle { w: 2, h: 2 };
    
        assert!(!r2.can_hold(&r1));
    }


    // custom panic messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Pepe");
        assert!(
            result.contains("Pepe"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }
    
    #[test]
    fn greeting_doesnt_contain_name() {
        let result = greeting("Pepe");
        assert!(
            result.contains("Luis"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }


    // should_panic 
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }


    // test with Result<T, E>
    #[test]
    fn test_result() -> Result<(), String> {
        if sum(2, 2) == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four"))
        }
    }


    #[test]
    #[ignore]
    fn ignore_test() {
        println!("abcde");
    }
}