use tokio::spawn;
use tokio_schedule::{every, Job};

#[tokio::main]
async fn main(){
    println!("[ºº]");
    let job = every(5).seconds()
        .perform(|| async { println!(" > ") });
    spawn(job);

    // Keep the application running
    std::future::pending::<()>().await;
}