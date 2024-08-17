use add_one;
use rand;

fn main() {
    let n = 10;
    println!(
        "Hello World! {} plus one is {}",
        n,
        add_one::add_one(n)
    );
}
