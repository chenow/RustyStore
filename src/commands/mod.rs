mod parser;
pub enum CommandName {
    PING,
}

pub struct Command {
    name: CommandName,
    run: fn(&[String]) -> Result<Vec<u8>, Vec<u8>>,
}

const WHITELISTED_COMMANDS: [&str; 1] = ["CLIENT"];

fn ping_command(_args: &[String]) -> Result<Vec<u8>, Vec<u8>> {
    // Return the correct Redis PONG response as a vector of bytes
    Ok(b"+PONG\r\n".to_vec())
}

pub const PING: Command = Command {
    name: CommandName::PING,
    run: ping_command,
};
pub fn handle_client_payload(payload: &[u8]) -> Vec<u8> {
    // Parse the payload into a command
    let command = match parser::parse_commands(payload) {
        Ok(mut commands) => {
            // Check if the command is whitelisted
            let command = commands.remove(0);
            if WHITELISTED_COMMANDS.contains(&command[0].as_str()) {
                command
            } else if command[0].to_uppercase() == "PING" {
                return (PING.run)(&command[1..]).unwrap();
            } else {
                let error = format!("-unknown command {}\r\n", command[0]);
                return error.as_bytes().to_vec();
            }
        }
        Err(e) => {
            eprintln!("Failed to parse command: {}", e);
            return b"-ERR failed to parse command\r\n".to_vec();
        }
    };

    // Get the command name and arguments
    let command_name = command[0].to_uppercase();
    let args = command[1..].to_vec();

    // Get the command from the command name
    let command = match command_name.as_str() {
        "PING" => PING,
        _ => {
            eprintln!("Unknown command: {}", command_name);
            return b"-ERR unknown command\r\n".to_vec();
        }
    };

    // Get the result from the command, and return the response bytes
    let response = match (command.run)(&args) {
        Ok(resp) => resp,
        Err(e) => e,
    };

    response
}
