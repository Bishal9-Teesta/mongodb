use crate::structure::photo::Photo;

pub async fn get_photo() -> Vec<Photo> {
    let photos = reqwest::get("https://jsonplaceholder.typicode.com/photos").await.unwrap().text().await.unwrap();
    // println!("Response: {:#?}", photos);

    let data: Vec<Photo> = serde_json::from_str(photos.as_str()).unwrap();
    // println!("Photo List: {:#?}", data);

    data
}
