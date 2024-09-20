mod utils;
use tokio;

use utils::get_redis_connection;

#[tokio::test]
async fn test_redis_ping() {
    let mut con = get_redis_connection()
        .await
        .expect("Failed to connect to Redis");

    let pong: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .expect("Failed to execute PING");
    assert_eq!(pong, "PONG");
}

#[tokio::test]
async fn test_two_clients_concurrently() {
    let client1 = tokio::spawn(async {
        let mut con = get_redis_connection()
            .await
            .expect("Client 1: Failed to connect to Redis");
        let pong: String = redis::cmd("PING")
            .query_async(&mut con)
            .await
            .expect("Client 1: Failed to execute PING");
        assert_eq!(pong, "PONG");
    });

    let client2 = tokio::spawn(async {
        let mut con = get_redis_connection()
            .await
            .expect("Client 2: Failed to connect to Redis");
        let pong: String = redis::cmd("PING")
            .query_async(&mut con)
            .await
            .expect("Client 2: Failed to execute PING");
        assert_eq!(pong, "PONG");
    });

    // Wait for both clients to finish
    let _ = tokio::try_join!(client1, client2).expect("Both clients should execute successfully");
}

#[tokio::test]
async fn test_one_client_two_commands() {
    let mut con = get_redis_connection()
        .await
        .expect("Failed to connect to Redis");

    let pong: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .expect("Failed to execute PING");
    assert_eq!(pong, "PONG");

    let pong: String = redis::cmd("PING")
        .query_async(&mut con)
        .await
        .expect("Failed to execute PING");
    assert_eq!(pong, "PONG");
}
