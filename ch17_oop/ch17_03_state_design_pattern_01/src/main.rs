//////////////////////////////
// 17.3. State Design Pattern
// https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html
//////////////////////////////

use ch17_03_state_design_pattern_01::Post;

fn main() {
    let mut post = Post::new();
    
    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());
    
    post.request_review();
    assert_eq!("", post.content());
    
    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());
}
