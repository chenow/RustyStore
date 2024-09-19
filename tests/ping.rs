use redis::Connection;

fn get_redis_connection() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
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
