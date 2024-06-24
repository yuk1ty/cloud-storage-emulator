use chrono::{DateTime, Local};

use crate::storage::StorageBucketAttr;

#[derive(Debug, Clone)]
pub struct Bucket {
    pub name: String,
    pub versioning: bool,
    pub default_event_based_hold: bool,
    pub time_created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl From<StorageBucketAttr> for Bucket {
    fn from(value: StorageBucketAttr) -> Self {
        let StorageBucketAttr {
            name,
            versioning,
            default_event_based_hold,
            time_created,
            updated,
        } = value;
        Self {
            name,
            versioning,
            default_event_based_hold,
            time_created,
            updated,
        }
    }
}
