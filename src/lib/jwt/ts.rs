use chrono::{Duration, Utc};

pub fn now_ts() -> i64 {
    Utc::now().timestamp()
}

pub fn exp_ts() -> i64 {
    now_ts() + Duration::days(7).num_seconds()
}
