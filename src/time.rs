use chrono::NaiveDateTime;
use prost_types::Timestamp;

pub trait ToProtoTimestamp {
    fn to_proto(self) -> Timestamp;
}

impl ToProtoTimestamp for NaiveDateTime {
    fn to_proto(self) -> Timestamp {
        Timestamp {
            seconds: self.and_utc().timestamp(),
            nanos: 0,
        }
    }
}

pub trait ToDateTime {
    fn to_datetime(self) -> Option<DateTime>;
}

impl ToDateTime for Timestamp {
    fn to_datetime(self) -> Option<DateTime> {
        DateTime::from_timestamp_secs(self.seconds)
    }
}

pub type DateTime = chrono::DateTime<chrono::Utc>;
