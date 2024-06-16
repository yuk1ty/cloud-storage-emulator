use crate::{kernel::bucket::Bucket, storage::Storage};

pub async fn get_all_buckets(storage: Storage) -> Vec<Bucket> {
    storage.all().await;
    todo!()
}
