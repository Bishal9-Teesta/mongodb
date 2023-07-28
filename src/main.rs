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
    // println!("Photo List: {:#?}", photo_list);
    let post_list = data::post::get_post().await;
    // println!("Post List: {:#?}", post_list);

    let database= DB_CONNECTION_POOL.lock().unwrap().clone().unwrap();

    // Photos collection
    let collection_list = database.list_collection_names(None).await.unwrap();
    // println!("Collection List: {:#?}", collection_list);

    let mut collection_exist = false;
    for collection_name in &collection_list {
        if *collection_name == "photos" {
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
    println!("Successfully inserted photos list.");

    // Posts
    collection_exist = false;
    for collection_name in &collection_list {
        if *collection_name == "posts" {
            collection_exist = true
        }
    }
    if collection_exist {
        println!("Collection 'posts' do exist.")
    } else {
        println!("Collection 'posts' does not exist.");
        database.create_collection("posts", None).await.expect("Collection 'post' creation error.");
    }

    database.collection("posts").insert_many(post_list, None).await.expect("Error Inserting Posts List.");
    println!("Successfully inserted posts list.");
}
