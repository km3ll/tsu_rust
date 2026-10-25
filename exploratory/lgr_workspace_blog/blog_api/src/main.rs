use blog_shared::Post;

fn main() {
    println!(" > main-api:");
    let post = Post::new("Post on the API!".to_owned(), "Let's Get Rusty!".to_owned());
    println!(" > main-api: {post:?}");
}
