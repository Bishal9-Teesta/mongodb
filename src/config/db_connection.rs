
pub async fn mongodb_client() {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    let db_list = client.list_database_names(None, None).await.unwrap();
    // println!("DB List: {:#?}", db_list);

    let mut database_exist = false;
    for single_database in db_list {
        if single_database == "json_placeholder".to_string() {
            database_exist = true
        }
    }

    if !database_exist {
        println!("Database 'json_placeholder' do not exist.")
    } else {
        println!("Database 'json_placeholder' exist.")
    }
    let database = client.database("json_placeholder");
    crate::global::store::set_global(database);
}
