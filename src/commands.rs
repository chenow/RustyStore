use crate::db::Db;

pub async fn process_command(command: String, db: &Db) -> String {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["PING"] => "+PONG\r\n".to_string(),

        ["SET", key, value] => {
            let mut db = db.write().await;
            db.insert(key.to_string(), value.to_string());
            "+OK\r\n".to_string()
        }

        ["GET", key] => {
            let db = db.read().await;
            match db.get(&key.to_string()) {
                Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                _ => "$-1\r\n".to_string(),
            }
        }

        _ => "-ERR unknown command\r\n".to_string(),
    }
}
