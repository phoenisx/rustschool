// This Binary is to show how State Pattern works with Rust lang
// to support OOP

use oops::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    println!("{}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{}", post.content())
}
