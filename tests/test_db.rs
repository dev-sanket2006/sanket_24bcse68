// tests/test_db.rs

use intelligent_deduplicator::db::connect_to_mongo;

#[tokio::test]
async fn test_db_connection() {
    let client = connect_to_mongo().await;
    let db_names = client.list_database_names(None, None).await.unwrap();
    assert!(db_names.len() >= 0); // At least connects successfully
}
