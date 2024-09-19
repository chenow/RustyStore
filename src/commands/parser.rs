use std::str;

pub fn parse_commands(data: &[u8]) -> Result<Vec<Vec<String>>, String> {
    let mut idx = 0;
    let mut commands = Vec::new();

    while idx < data.len() {
        // Skip any whitespace or newlines (if necessary)
        // For RESP, commands start with '*'
        if data[idx] == b'*' {
            let command = parse_array(data, &mut idx)?;
            commands.push(command);
        } else {
            return Err(format!(
                "Expected '*', found '{}' at position {}",
                data[idx] as char, idx
            ));
        }
    }

    Ok(commands)
}

fn parse_array(data: &[u8], idx: &mut usize) -> Result<Vec<String>, String> {
    // Expecting '*'
    *idx += 1;

    // Parse the number of elements
    let num_elements = parse_number(data, idx)?;

    // Expect CRLF
    expect_crlf(data, idx)?;

    let mut elements = Vec::new();

    for _ in 0..num_elements {
        if *idx >= data.len() {
            return Err("Unexpected end of data while parsing array elements".to_string());
        }

        match data[*idx] {
            b'$' => {
                let s = parse_bulk_string(data, idx)?;
                elements.push(s);
            }
            _ => {
                return Err(format!(
                    "Expected '$' in array element, found '{}' at position {}",
                    data[*idx] as char, *idx
                ));
            }
        }
    }

    Ok(elements)
}

fn parse_bulk_string(data: &[u8], idx: &mut usize) -> Result<String, String> {
    // Expecting '$'
    *idx += 1;

    // Parse the length of the bulk string
    let length = parse_number(data, idx)?;

    // Expect CRLF
    expect_crlf(data, idx)?;

    // Ensure there's enough data
    if *idx + length > data.len() {
        return Err("Unexpected end of data while reading bulk string".to_string());
    }

    // Extract the string
    let s = str::from_utf8(&data[*idx..*idx + length])
        .map_err(|e| format!("Invalid UTF-8 in bulk string: {}", e))?
        .to_string();

    *idx += length;

    // Expect CRLF
    expect_crlf(data, idx)?;

    Ok(s)
}

fn parse_number(data: &[u8], idx: &mut usize) -> Result<usize, String> {
    let start = *idx;

    while *idx < data.len() && data[*idx] != b'\r' {
        *idx += 1;
    }

    if *idx >= data.len() {
        return Err("Unexpected end of data while parsing number".to_string());
    }

    let s = str::from_utf8(&data[start..*idx])
        .map_err(|e| format!("Invalid UTF-8 in number: {}", e))?;

    let num = s
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse number '{}': {}", s, e))?;

    Ok(num)
}

fn expect_crlf(data: &[u8], idx: &mut usize) -> Result<(), String> {
    if *idx + 1 >= data.len() {
        return Err("Unexpected end of data while expecting CRLF".to_string());
    }

    if data[*idx] == b'\r' && data[*idx + 1] == b'\n' {
        *idx += 2;
        Ok(())
    } else {
        Err(format!(
            "Expected CRLF at position {}, found '{}{}'",
            idx,
            data[*idx] as char,
            data[*idx + 1] as char
        ))
    }
}
