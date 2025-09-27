use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

pub fn get_now_millis() -> Result<i64, SystemTimeError> {
    let now = SystemTime::now();
    let elapsed = now.duration_since(UNIX_EPOCH)?;
    
    Ok(elapsed.as_millis() as i64)
}