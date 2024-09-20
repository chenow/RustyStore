use tokio;
mod utils;

use utils::get_redis_connection;

#[tokio::test]
async fn test_set_and_get_command() {
    let mut con = get_redis_connection()
        .await
        .expect("Failed to connect to Redis");

    let set: () = redis::cmd("SET")
        .arg("my_key")
        .arg("my_value")
        .query_async(&mut con)
        .await
        .expect("Failed to execute SET");

    assert_eq!(set, ());

    let value: String = redis::cmd("GET")
        .arg("my_key")
        .query_async(&mut con)
        .await
        .expect("Failed to execute GET");

    assert_eq!(value, "my_value");
}

#[tokio::test]
async fn test_get_command_no_value() {
    let mut con = get_redis_connection()
        .await
        .expect("Failed to connect to Redis");

    let value: Option<String> = redis::cmd("GET")
        .arg("not_existing_key")
        .query_async(&mut con)
        .await
        .expect("Failed to execute GET");

    assert_eq!(value, None);
}

#[tokio::test]
async fn test_get_set_concurrently() {
    let client1 = tokio::spawn(async {
        let mut con = get_redis_connection()
            .await
            .expect("Failed to connect to Redis");

        let _: () = redis::cmd("SET")
            .arg("my_key")
            .arg("my_value")
            .query_async(&mut con)
            .await
            .expect("Failed to execute SET");
    });

    let client2 = tokio::spawn(async {
        let mut con = get_redis_connection()
            .await
            .expect("Failed to connect to Redis");

        let value: String = redis::cmd("GET")
            .arg("my_key")
            .query_async(&mut con)
            .await
            .expect("Failed to execute GET");

        assert_eq!(value, "my_value");
    });

    let _ = tokio::try_join!(client1, client2).expect("Both clients should execute successfully");
}
