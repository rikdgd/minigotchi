use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now_millis() -> i64 {
    let now = SystemTime::now();
    let elapsed = now.duration_since(UNIX_EPOCH)
        .expect("Failed to calculate now in millis, UNIX_EPOCH was lower then system time");
    
    elapsed.as_millis() as i64
}