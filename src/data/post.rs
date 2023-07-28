use crate::structure::post::Post;

pub async fn get_post() -> Vec<Post> {
    let posts = reqwest::get("https://jsonplaceholder.typicode.com/posts").await.unwrap().text().await.unwrap();
    // println!("Response: {:#?}", posts);

    let data: Vec<Post> = serde_json::from_str(posts.as_str()).unwrap();
    // println!("Photo List: {:#?}", data);

    data
}
