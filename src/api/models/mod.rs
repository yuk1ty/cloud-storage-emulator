use bucket::BucketResponse;
use serde::Serialize;

use crate::kernel::bucket::Bucket;

pub mod bucket;

#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
    pub kind: ListKind,
    pub items: Vec<T>,
    pub prefixes: Vec<String>,
}

impl From<Vec<Bucket>> for ListResponse<BucketResponse> {
    fn from(buckets: Vec<Bucket>) -> Self {
        ListResponse {
            kind: ListKind::Buckets,
            items: buckets.into_iter().map(|bucket| bucket.into()).collect(),
            prefixes: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
pub enum ListKind {
    #[serde(rename = "storage#buckets")]
    Buckets,
    #[allow(dead_code)]
    #[serde(rename = "storage#objects")]
    Objects,
}

// TODO: need to remove `Default` trait here
#[derive(Debug, Default, Serialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "storage#bucket")]
    Bucket,
    #[allow(dead_code)]
    #[serde(rename = "storage#object")]
    Object,
}
