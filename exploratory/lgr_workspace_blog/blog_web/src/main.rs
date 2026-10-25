use blog_shared::Post;

fn main() {
    println!(" > main-web:");
    let post = Post::new("Post on the WEB!".to_owned(), "Let's Get Rusty!".to_owned());
    println!(" > main-web: {post:?}");
}
