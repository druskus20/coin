//use crate::errors::Error;
//use chrono::DateTime;
//use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

type SDb = surrealdb::Surreal<surrealdb::engine::remote::ws::Ws>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    pub title: String,
    pub amount: u32,
}

pub(super) struct Db {}

impl Db {
    pub(super) async fn try_init() -> Self {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await.unwrap();

        db.use_ns("coin").use_db("coin").await.unwrap();
        Self {}
    }
}
