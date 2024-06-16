use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::kernel::bucket::Bucket;

type InnerStorage = BTreeMap<String, Bucket>;

#[derive(Clone)]
pub struct Storage(Arc<Mutex<InnerStorage>>);

impl Storage {
    pub fn new() -> Self {
        Storage(Arc::new(Mutex::new(BTreeMap::new())))
    }

    pub async fn all(&self) -> Vec<&Bucket> {
        todo!()
    }
}
