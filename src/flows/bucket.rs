use std::convert::Infallible;

use crate::{
    api::models::bucket::CreateBucket,
    kernel::bucket::Bucket,
    libs::errors::{AppResult, Errors},
    repositories::bucket::{create_bucket, get_all_buckets},
    storage::{Storage, StorageBucketAttr},
};

pub async fn list(storage: Storage) -> Result<Vec<Bucket>, Infallible> {
    Ok(get_all_buckets(storage)
        .await
        .into_iter()
        .map(Bucket::from)
        .collect())
}

pub async fn create_new_bucket(storage: Storage, event: CreateBucket) -> AppResult<Bucket, Errors> {
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
    create_bucket(storage, attr).await.map(Bucket::from)
}
