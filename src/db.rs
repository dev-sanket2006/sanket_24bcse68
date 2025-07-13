use mongodb::Client;

pub async fn connect_to_mongo() -> Client {
    Client::with_uri_str("mongodb://localhost:27017").await.expect("Failed to connect to MongoDB")
}
