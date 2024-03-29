#[tokio::main]
async fn main() {
    let coin = coin_core::Coin::try_new().await.unwrap();
    coin.add_expense(99).await.unwrap();
    let exs = coin.get_expenses().await.unwrap();
    dbg!(exs);
    todo!("This will become a CLI application mimicking the tauri frontend")
}
