use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

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
    pub time_created: DateTime<Local>,
    pub updated: DateTime<Local>,
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
            id: value.name.clone(),
            name: value.name,
            time_created: value.time_created,
            updated: value.updated,
            default_event_based_hold: value.default_event_based_hold,
            versioning: BucketVersioning {
                enabled: value.versioning,
            },
            // TODO
            location: "us".to_string(),
            storage_class: "STANDARD".to_string(),
            project_number: "1".to_string(),
            metageneration: "1".to_string(),
            etag: "tag".to_string(),
            location_type: "region".to_string(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BucketVersioning {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct CreateBucket {
    #[garde(pattern("^[a-zA-Z0-9][a-zA-Z0-9._-]*[a-zA-Z0-9]$"))]
    pub name: String,
    #[garde(skip)]
    pub versioning: Option<BucketVersioning>,
    #[garde(skip)]
    #[serde(default)]
    pub default_event_based_hold: bool,
}
