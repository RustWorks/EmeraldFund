use chrono::Utc;

pub type TimeStamp = u64;

pub fn get_unix_time() -> TimeStamp {
    let now = Utc::now();
    now.timestamp().try_into().unwrap()
}
