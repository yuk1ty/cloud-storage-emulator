use std::convert::Infallible;

use crate::{
    api::models::bucket::CreateBucket,
    kernel::bucket::Bucket,
    libs::errors::Errors,
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

pub async fn create_new_bucket(storage: Storage, event: CreateBucket) -> Result<Bucket, Errors> {
    let CreateBucket {
        name, versioning, .. // TODO handling other fields
    } = event;
    let attr = StorageBucketAttr {
        name,
        versioning: versioning.enabled,
        time_created: chrono::Utc::now().naive_utc(),
        updated: chrono::Utc::now().naive_utc(),
    };
    create_bucket(storage, attr).await.map(Bucket::from)
}
