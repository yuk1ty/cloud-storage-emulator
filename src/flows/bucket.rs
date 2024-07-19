use std::convert::Infallible;

use crate::{
    api::models::bucket::CreateBucket,
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
    Ok(storage.read(bucket_name).await)
}

pub async fn create_new_bucket(
    storage: Storage,
    event: CreateBucket,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.create_bucket(event.into()).await
}
