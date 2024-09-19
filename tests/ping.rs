use redis::Connection;
use tokio::task;

fn get_redis_connection() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

#[test]
fn test_redis_ping() {
    let mut con = get_redis_connection().expect("Failed to connect to Redis");

    let pong: String = redis::cmd("PING")
        .query(&mut con)
        .expect("Failed to execute PING");
    assert_eq!(pong, "PONG");
}

#[test]
fn test_redis_two_pings() {
    let mut con = get_redis_connection().expect("Failed to connect to Redis");

    // Send first PING command
    let pong1: String = redis::cmd("PING")
        .query(&mut con)
        .expect("Failed to execute first PING");
    assert_eq!(pong1, "PONG");

    // Send second PING command using the same connection
    let pong2: String = redis::cmd("PING")
        .query(&mut con)
        .expect("Failed to execute second PING");
    assert_eq!(pong2, "PONG");
}

#[tokio::test]
async fn test_redis_ping_two_clients() {
    // Spawn the first client task
    let handle1 = task::spawn(async {
        let mut con = get_redis_connection().expect("Failed to connect client 1");
        let pong: String = redis::cmd("PING")
            .query(&mut con)
            .expect("Failed to execute PING from client 1");
        assert_eq!(pong, "PONG");
    });

    // Spawn the second client task
    let handle2 = task::spawn(async {
        let mut con = get_redis_connection().expect("Failed to connect client 1");
        let pong: String = redis::cmd("PING")
            .query(&mut con)
            .expect("Failed to execute PING from client 1");
        assert_eq!(pong, "PONG");
    });
    // Wait for both tasks to complete
    let _ = tokio::join!(handle1, handle2);
}

#[tokio::test]
async fn test_redis_not_known_command() {
    let mut con = get_redis_connection().expect("Failed to connect to Redis");

    let pong: String = redis::cmd("NOT_A_COMMAND")
        .query(&mut con)
        .expect("Failed to execute NOT_A_COMMAND");
    assert_eq!(pong, "(error) unknown command 'NOT_A_COMMAND'");
}
