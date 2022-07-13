use app::run;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //match run() {
    //    Ok(x) => x,
    //    Err(_) => println!("Failure"),
    //};
    run().await;
    Ok(())
}

//fn main() {
//    run();
//}
