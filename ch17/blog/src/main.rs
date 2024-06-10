use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    port.request_review();
    assert_eq!("", post.content());

    port.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}