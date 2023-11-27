// use serde::{Deserialize, Serialize};
use mini_redis::{client, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;

// async fn print() {
//     println!("Hello, world!");
// }

#[tokio::main]
async fn main() {
    // let f = print();
    // f.await;
    let mut client = Arc::new(AsyncMutex::new(client::connect("127.0.0.1:6379").await.unwrap()));
    let data1 = client.lock().await.get("data1").await.unwrap();
    tokio::spawn(async move {
        client.lock().await.set("hello", "world".into()).await.unwrap();
    });
    // let data1 = client.get("data1").await.unwrap();
    // println!("data1 = {:?}", data1);
}