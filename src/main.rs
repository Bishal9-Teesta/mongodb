use std::sync::Mutex;
use once_cell::sync::Lazy;

mod config;
mod structure;
mod data;
mod global;

static DB_CONNECTION_POOL: Lazy<Mutex<Option<mongodb::Database>>> = Lazy::new(|| { Mutex::new(None) });

#[tokio::main]
async fn main() {

    config::db_connection::mongodb_client().await;
    let photo_list = data::photo::get_photo().await;

    let database= DB_CONNECTION_POOL.lock().unwrap().clone().unwrap();
    // println!("Photo List: {:#?}", photo_list);

    let collection_list = database.list_collection_names(None).await.unwrap();
    // println!("Collection List: {:#?}", collection_list);

    let mut collection_exist = false;
    for collection_name in collection_list {
        if collection_name == "photos" {
            collection_exist = true
        }
    }
    if collection_exist {
        println!("Collection 'photos' do exist.")
    } else {
        println!("Collection 'photos' does not exist.");
        database.create_collection("photos", None).await.expect("Collection 'photo' creation error.");
    }

    database.collection("photos").insert_many(photo_list, None).await.expect("Error Inserting Photos List.");
    println!("Successfully inserted photos list.")
}
