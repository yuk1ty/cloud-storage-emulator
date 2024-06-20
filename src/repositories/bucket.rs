use crate::{kernel::bucket::Bucket, storage::Storage};

pub async fn get_all_buckets(storage: Storage) -> Vec<Bucket> {
    storage
        .read_all()
        .await
        .into_iter()
        .map(|b| b.into())
        .collect()
}
