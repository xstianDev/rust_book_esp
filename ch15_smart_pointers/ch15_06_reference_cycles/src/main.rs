//////////////////////////////
// 15.6. Reference Cylces
// https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html
//////////////////////////////

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use ch15_06_reference_cycles::List::{Cons, Nil};
use ch15_06_reference_cycles::Node;

fn memory_leak() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // "b" referencia "a"
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    
    // "a" referencia "b"
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Stack             Heap
    //   a    ->  'a  |  5 | 'b |
    //   b    ->  'b  | 10 | 'a |

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Esta línea causa overflow del stack
    println!("a next item = {:?}", a.tail());


    /* Cuando acabe la función, se limpiará "b" en el stack,
     * pero no en el heap ya que tiene una referencia de 'a.
     * 
     * Después se limpiará "a" en el stack,
     * pero no en el heap ya que tiene una referencia de 'b.
     */

    // Stack             Heap
    //   _    ->  '_  |  5 | 'b |
    //   _    ->  '_  | 10 | 'a |
}

fn weak_pointer() {
    // Weak: versión de Rc que toma una referencia sin tomar ownership.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn weak_and_strong_counts() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    memory_leak();
    weak_pointer();
    weak_and_strong_counts();
}
