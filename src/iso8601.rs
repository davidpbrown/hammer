//! Format timestamps as iso8601
use chrono::prelude::{DateTime, Utc};

pub fn iso8601(st: &std::time::SystemTime) -> String {
//! 	example output: 2021-08-28_13:37+12345
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%Y-%m-%d_%H:%M+%S%3f UTC"))
}

