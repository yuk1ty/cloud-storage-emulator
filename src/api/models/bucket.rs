use chrono::NaiveDateTime;
use serde::Serialize;

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

#[derive(Debug, Default, Serialize)]
pub struct BucketVersioning {
    pub enabled: bool,
}
