mod db;
//mod errors;

//use errors::Result;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::console;

pub struct Coin {
    db: db::Db,
}

impl Coin {
    pub async fn try_new() -> Coin {
        let db = db::Db::try_init().await;
        Self { db }
    }
}

#[wasm_bindgen]
pub async fn run() -> std::result::Result<(), JsError> {
    let coin = Coin::try_new().await;
    Ok(())
}
