use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_time_millis() {
        let before = current_time_millis();
        std::thread::sleep(Duration::from_millis(1));
        let after = current_time_millis();
        assert_ne!(before, after);
        assert!(after > before);
    }
}