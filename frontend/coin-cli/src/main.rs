#[tokio::main]
async fn main() {
    coin_core::init().await.unwrap();
}
