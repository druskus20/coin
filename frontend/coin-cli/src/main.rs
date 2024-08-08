use coin_core::{
    currency::Currency,
    expense::{Amount, Expense},
};

pub fn create_test_expense() -> Expense {
    Expense {
        id: 1,
        name: "Test Expense".to_string(),
        desc: "Test Description".to_string(),
        datetime: chrono::Utc::now(),
        category: "Test Category".to_string(),
        amount: Amount {
            id: 0,
            datetime: chrono::Utc::now(),
            base_currency: Currency::USD,
            currency: Currency::USD,
            amount_from_base: 100.0,
            amount_to_base: 100.0,
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), coin_core::error::CoinError> {
    let r = coin_core::init().await?;
    let e = create_test_expense();
    coin_core::add_expense(&r, e).await?;
    Ok(())
}
