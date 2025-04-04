use samson::I2PSamClient;
use tokio;

#[tokio::test]
async fn test_create_session() {
    let client = I2PSamClient::new("127.0.0.1".to_string(), 7656);
    match client.create_session().await {
        Ok(destination) => {
            assert!(!destination.is_empty());
            println!("Created session with destination: {}", destination);
        }
        Err(e) => {
            panic!("Failed to create session: {}", e);
        }
    }
}
