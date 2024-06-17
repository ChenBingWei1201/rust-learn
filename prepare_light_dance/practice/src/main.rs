use sqlx::{MySql, MySqlPool, Pool};
use std::env::var;
#[derive(Debug)]
struct GQLEntry {
    id: i32,
    name: String,
}

impl From<Entry> for GQLEntry {
    fn from(value: Entry) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    
    }
}

struct Entry {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main () {
    dotenv::dotenv().ok();
    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pool = MySqlPool::connect(&database_url)
        .await
        .unwrap();

    let result = sqlx::query!(
        r#"
            INSERT INTO Table1 (name) VALUES (?);
        "#,
        "test"
    )
    .execute(&pool)
    .await
    .unwrap();
    println!("{:?}", result);
}
