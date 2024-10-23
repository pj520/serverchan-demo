use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let key = env::var("SENDKEY").unwrap();
    let ret = server_chan::sc_send("主人服务器宕机了 via Rust".to_string(), "第一行\n\n第二行".to_string(), key).await?;
    println!("{}", ret);
    Ok(())
}
