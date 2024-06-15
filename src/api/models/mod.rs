use serde::Serialize;

pub mod bucket;

#[derive(Debug, Serialize)]
pub struct ListResponse<T: Serialize> {
    pub kind: ListKind,
    pub items: Vec<T>,
    pub prefixes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum ListKind {
    #[serde(rename = "storage#buckets")]
    Buckets,
    #[serde(rename = "storage#objects")]
    Objects,
}

// TODO: need to remove `Default` trait here
#[derive(Debug, Default, Serialize)]
pub enum Kind {
    #[default]
    #[serde(rename = "storage#bucket")]
    Bucket,
    #[serde(rename = "storage#object")]
    Object,
}
