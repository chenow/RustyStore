pub fn process_command(command: String) -> String {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["PING"] => "+PONG\r\n".to_string(),
        ["SET", _key, _value] => "+OK\r\n".to_string(),
        ["GET", _key] => "$-1\r\n".to_string(),
        _ => "-ERR unknown command\r\n".to_string(),
    }
}
