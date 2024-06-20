use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct StorageBucketAttr {
    pub name: String,
    pub versioning: bool,
    pub time_created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Debug, PartialEq)]
pub struct OnMemoryStorageBucket {
    pub attr: StorageBucketAttr,
    pub objects: BTreeMap<String, OnMemoryStorageObject>,
}

#[derive(Debug, PartialEq)]
pub struct OnMemoryStorageObject {
    // TODO: add attr
    pub data: Vec<u8>,
}

type InnerStorage = BTreeMap<String, Arc<OnMemoryStorageBucket>>;

#[derive(Clone)]
pub struct Storage(Arc<RwLock<InnerStorage>>);

impl Storage {
    pub fn new() -> Self {
        Storage(Arc::new(RwLock::new(BTreeMap::new())))
    }

    pub async fn read_all(&self) -> Vec<StorageBucketAttr> {
        let storage = self.0.read().unwrap();
        let buckets = storage
            .values()
            .cloned()
            .collect::<Vec<Arc<OnMemoryStorageBucket>>>();
        buckets.iter().map(|b| b.attr.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        sync::{Arc, RwLock},
    };

    use rstest::rstest;

    use crate::storage::{OnMemoryStorageBucket, Storage, StorageBucketAttr};

    #[rstest]
    #[tokio::test]
    async fn return_all_passed_buckets() {
        let attr1 = StorageBucketAttr {
            name: "test_bucket_1".to_string(),
            versioning: false,
            time_created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
        };
        let bucket1 = OnMemoryStorageBucket {
            attr: attr1.clone(),
            objects: BTreeMap::new(),
        };
        let attr2 = StorageBucketAttr {
            name: "test_bucket_2".to_string(),
            versioning: false,
            time_created: chrono::Utc::now().naive_utc(),
            updated: chrono::Utc::now().naive_utc(),
        };
        let bucket2 = OnMemoryStorageBucket {
            attr: attr2.clone(),
            objects: BTreeMap::new(),
        };

        let storage = Storage(Arc::new(RwLock::new(
            vec![
                ("test_bucket_1".to_string(), Arc::new(bucket1)),
                ("test_bucket_2".to_string(), Arc::new(bucket2)),
            ]
            .into_iter()
            .collect(),
        )));
        // assert_eq_unordered!(storage.read_all().await, vec![attr1, attr2]);
        assert_eq!(storage.read_all().await, vec![attr1, attr2]);
    }
}
