use crate::{
    libs::errors::{AppResult, Errors},
    storage::{Storage, StorageBucketAttr},
};

pub async fn get_all_buckets(storage: Storage) -> Vec<StorageBucketAttr> {
    storage.read_all().await.into_iter().collect()
}

pub async fn create_bucket(
    storage: Storage,
    attr: StorageBucketAttr,
) -> AppResult<StorageBucketAttr, Errors> {
    storage.create_bucket(attr).await
}
