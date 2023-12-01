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

    let res = sqlx::query_as!(
        Entry, 
        "SELECT * FROM Table1 WHERE id = ?;",
        0
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let res: Vec<GQLEntry> = res.into_iter().map(|x| x.into()).collect();
    println!("{:?}", res);
}
