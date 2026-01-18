use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};

/// SystemTime to String
pub fn system_time_to_string(system_time: SystemTime) -> String {
    let duration_since_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime earlier than UNIX_EPOCH");

    // Build a chrono DateTime<Local> from the seconds + nanoseconds
    let datetime: DateTime<Local> = DateTime::from_timestamp(
        duration_since_epoch.as_secs() as i64,
        duration_since_epoch.subsec_nanos(),
    )
    .expect("Invalid timestamp")
    .with_timezone(&Local);
    let x = datetime.to_string();
    x
}
