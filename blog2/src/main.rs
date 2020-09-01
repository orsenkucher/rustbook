use blog2::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("text");
    let mut post = post.request_review();
    post.approve();
    post.approve();
    let post = post.publish().unwrap();
    assert_eq!(post.contents(), "text");
}
