use chrono::NaiveDateTime;
use serde::Serialize;

use crate::kernel::bucket::Bucket;

use super::Kind;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketResponse {
    pub kind: Kind,
    pub id: String,
    pub default_event_based_hold: bool,
    pub name: String,
    pub versioning: BucketVersioning,
    pub time_created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub location: String,
    pub storage_class: String,
    pub project_number: String,
    pub metageneration: String,
    pub etag: String,
    pub location_type: String,
}

impl From<Bucket> for BucketResponse {
    fn from(value: Bucket) -> Self {
        BucketResponse {
            kind: Kind::Bucket,
            name: value.name,
            time_created: value.time_created,
            updated: value.updated,
            versioning: BucketVersioning {
                enabled: value.versioning,
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct BucketVersioning {
    pub enabled: bool,
}
