use crate::types::Value;

pub fn read_file(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("__io_read_file expects 1 argument".to_string());
    }
    let path_val = &args[0];
    let path = match path_val {
        Value::String(s) => s,
        _ => return Err("path must be a string".to_string()),
    };
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(format!("Failed to read file {}: {}", path, e)),
    }
}

pub fn write_file(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("__io_write_file expects 2 arguments".to_string());
    }
    let path_val = &args[0];
    let content_val = &args[1];
    let path = match path_val {
        Value::String(s) => s,
        _ => return Err("path must be a string".to_string()),
    };
    let content = match content_val {
        Value::String(s) => s,
        _ => return Err("content must be a string".to_string()),
    };
    match std::fs::write(path, content) {
        Ok(_) => Ok(Value::Void),
        Err(e) => Err(format!("Failed to write file {}: {}", path, e)),
    }
}
