use crate::types::Value;
use std::sync::{Arc, Mutex};

pub fn split(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("__str_split expects 2 arguments".to_string());
    }
    let text_val = &args[0];
    let delim_val = &args[1];
    let text = match text_val {
        Value::String(s) => s,
        _ => return Err("text must be a string".to_string()),
    };
    let delim = match delim_val {
        Value::String(s) => s,
        _ => return Err("delimiter must be a string".to_string()),
    };

    let parts: Vec<Value> = text
        .split(delim)
        .map(|s| Value::String(s.to_string()))
        .collect();
    Ok(Value::List(Arc::new(Mutex::new(parts))))
}

pub fn trim(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err("__str_trim expects 1 argument".to_string());
    }
    let text_val = &args[0];
    let text = match text_val {
        Value::String(s) => s,
        _ => return Err("text must be a string".to_string()),
    };
    Ok(Value::String(text.trim().to_string()))
}
