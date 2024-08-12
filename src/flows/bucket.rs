use std::convert::Infallible;

use crate::{
    api::models::bucket::{InsertBucket, UpdateBucket},
    libs::errors::{AppResult, Errors},
    storage::{Storage, StorageBucketAttr},
};

pub async fn list(storage: Storage) -> Result<Vec<StorageBucketAttr>, Infallible> {
    Ok(storage.read_all().await.into_iter().collect())
}

pub async fn find_bucket(
    storage: Storage,
    bucket_name: String,
) -> Result<Option<StorageBucketAttr>, Infallible> {
    Ok(storage.read(&bucket_name).await)
}

pub async fn create_new_bucket(
    storage: Storage,
    event: InsertBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.create_bucket(event.into()).await
}

pub async fn update_existing_bucket(
    storage: Storage,
    bucket_name: String,
    event: UpdateBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.update_bucket_attr(bucket_name, event.into()).await
}

pub async fn delete_bucket(
    storage: Storage,
    bucket_name: String,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.delete_bucket(bucket_name).await
}
