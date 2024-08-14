use std::convert::Infallible;

use crate::{
    api::models::bucket::{InsertBucket, UpdateBucket},
    libs::errors::{AppResult, Errors},
    storage::{BucketStorageExt, Storage, StorageBucketAttr},
};

pub async fn list(storage: Storage) -> Result<Vec<StorageBucketAttr>, Infallible> {
    Ok(storage.list().await.into_iter().collect())
}

pub async fn find_bucket(
    storage: Storage,
    bucket_name: String,
) -> Result<Option<StorageBucketAttr>, Infallible> {
    Ok(storage.get(&bucket_name).await)
}

pub async fn create_new_bucket(
    storage: Storage,
    event: InsertBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    let bucket_name = event.name.clone();
    storage.create(&bucket_name, event.into()).await
}

pub async fn update_existing_bucket(
    storage: Storage,
    bucket_name: String,
    event: UpdateBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.update(&bucket_name, event.into()).await
}

pub async fn delete_bucket(
    storage: Storage,
    bucket_name: String,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.delete(&bucket_name).await
}
