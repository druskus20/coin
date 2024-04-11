use coin_core::errors::Result;
use coin_core::{self, Coin};
use leptos::*;

fn main() -> Result<()> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::spawn_local(async {
        let coin = Coin::try_new().await.unwrap();
        coin.add_expense(99).await.unwrap();
        let expenses = coin.get_expenses().await.unwrap();

        for expense in expenses {
            log::info!("{:?}", expense);
        }
    });

    mount_to_body(|| {
        view! { <p>"Hello, world!"</p> }
    });

    Ok(())
}
