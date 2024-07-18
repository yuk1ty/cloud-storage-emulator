use std::convert::Infallible;

use crate::{
    api::models::bucket::CreateBucket,
    libs::errors::{AppResult, Errors},
    storage::{Storage, StorageBucketAttr},
};

pub async fn list(storage: Storage) -> Result<Vec<StorageBucketAttr>, Infallible> {
    Ok(storage.read_all().await.into_iter().collect())
}

pub async fn create_new_bucket(
    storage: Storage,
    event: CreateBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    let CreateBucket {
        name,
        versioning,
        default_event_based_hold,
    } = event;
    let attr = StorageBucketAttr {
        name,
        versioning: versioning.map_or(false, |v| v.enabled),
        default_event_based_hold,
        time_created: chrono::Local::now(),
        updated: chrono::Local::now(),
    };
    storage.create_bucket(attr).await
}
