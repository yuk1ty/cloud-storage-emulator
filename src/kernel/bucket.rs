use chrono::NaiveDateTime;

use crate::storage::StorageBucketAttr;

#[derive(Debug, Clone)]
pub struct Bucket {
    pub name: String,
    pub versioning: bool,
    pub time_created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl From<StorageBucketAttr> for Bucket {
    fn from(value: StorageBucketAttr) -> Self {
        let StorageBucketAttr {
            name,
            versioning,
            time_created,
            updated,
        } = value;
        Self {
            name,
            versioning,
            time_created,
            updated,
        }
    }
}
