use std::sync::Mutex;
use once_cell::sync::Lazy;

mod config;
mod structure;
mod data;
mod global;

static GLOBAL: Lazy<Mutex<i128>> = Lazy::new(|| { Mutex::new(0) });

#[tokio::main]
async fn main() {

    // data::photo::get_photo().await;
    // config::db_connection::mongodb_client().await;

    println!("GLOBAL: {}", global::store::get_global());
    global::store::set_global(10);
    println!("GLOBAL: {}", global::store::get_global());
    global::store::set_global(20);
    println!("GLOBAL: {}", global::store::get_global());
}
