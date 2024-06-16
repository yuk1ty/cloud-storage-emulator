use std::convert::Infallible;

use crate::{kernel::bucket::Bucket, repositories::bucket::get_all_buckets, storage::Storage};

pub async fn list(storage: Storage) -> Result<Vec<Bucket>, Infallible> {
    Ok(get_all_buckets(storage).await)
}
