use redis::aio::MultiplexedConnection;

pub async fn get_redis_connection() -> redis::RedisResult<MultiplexedConnection> {
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    client.get_multiplexed_tokio_connection().await
}
