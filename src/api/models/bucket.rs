use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::storage::{StorageBucketAttr, UpdateBucketAttr};

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

impl From<StorageBucketAttr> for BucketResponse {
    fn from(value: StorageBucketAttr) -> Self {
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
            location: value.location,
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
pub struct InsertBucket {
    #[garde(pattern("^[a-zA-Z0-9][a-zA-Z0-9._-]*[a-zA-Z0-9]$"))]
    pub name: String,
    #[garde(skip)]
    pub versioning: Option<BucketVersioning>,
    #[garde(skip)]
    #[serde(default)]
    pub default_event_based_hold: bool,
    // TODO enum
    #[garde(skip)]
    pub location: Option<String>,
}

impl From<InsertBucket> for StorageBucketAttr {
    fn from(event: InsertBucket) -> Self {
        let InsertBucket {
            name,
            versioning,
            default_event_based_hold,
            location,
        } = event;
        StorageBucketAttr {
            name,
            versioning: versioning.map_or(false, |v| v.enabled),
            location: location.unwrap_or_else(|| "US".to_string()),
            default_event_based_hold,
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        }
    }
}

#[derive(Debug, Deserialize, garde::Validate)]
pub struct UpdateBucket {
    #[garde(skip)]
    pub versioning: Option<BucketVersioning>,
    #[garde(skip)]
    #[serde(default)]
    pub default_event_based_hold: bool,
}

impl From<UpdateBucket> for UpdateBucketAttr {
    fn from(event: UpdateBucket) -> Self {
        let UpdateBucket {
            versioning,
            default_event_based_hold,
        } = event;
        UpdateBucketAttr {
            versioning: versioning.map(|v| v.enabled),
            default_event_based_hold,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Projection {
    Full,
    NoAcl,
}

/// Represents a request parameter for `get` bucket.
/// https://cloud.google.com/storage/docs/json_api/v1/buckets/get#parameters
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct GetBucketParams {
    if_metageneration_match: Option<u64>,
    if_metageneration_not_match: Option<u64>,
    projection: Option<Projection>,
}

#[derive(Debug, Deserialize, EnumString)]
pub enum PredefinedAcl {
    #[strum(serialize = "authenticatedRead")]
    AuthenticatedRead,
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "projectPrivate")]
    ProjectPrivate,
    #[strum(serialize = "publicRead")]
    PublicRead,
    #[strum(serialize = "publicReadWrite")]
    PublicReadWrite,
}

#[derive(Debug, Deserialize, EnumString)]
pub enum PredefinedDefaultObjectAcl {
    #[strum(serialize = "authenticatedRead")]
    AuthenticatedRead,
    #[strum(serialize = "bucketOwnerFullControl")]
    BucketOwnerFullControl,
    #[strum(serialize = "bucketOwnerRead")]
    BucketOwnerRead,
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "projectPrivate")]
    ProjectPrivate,
    #[strum(serialize = "publicRead")]
    PublicRead,
}

/// Represents a request parameter for `insert` bucket.
/// https://cloud.google.com/storage/docs/json_api/v1/buckets/insert#parameters
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct InsertBucketParams {
    project: String,
    enable_object_retention: Option<bool>,
    predefined_acl: Option<PredefinedAcl>,
    predefined_default_object_acl: Option<PredefinedDefaultObjectAcl>,
    projection: Option<Projection>,
}

/// Represents a request parameter for `update` bucket.
/// https://cloud.google.com/storge/docs/json_api/v1/buckets/update#parameters
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct UpdateBucketParams {
    if_metageneration_match: Option<u64>,
    if_metageneration_not_match: Option<u64>,
    predefined_acl: Option<PredefinedAcl>,
    predefined_default_object_acl: Option<PredefinedDefaultObjectAcl>,
    projection: Option<Projection>,
}
