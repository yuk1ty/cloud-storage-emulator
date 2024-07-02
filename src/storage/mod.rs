use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Local};

use crate::libs::errors::{AppResult, Errors};

#[derive(Debug, Clone, PartialEq)]
pub struct StorageBucketAttr {
    pub name: String,
    pub versioning: bool,
    pub default_event_based_hold: bool,
    pub time_created: DateTime<Local>,
    pub updated: DateTime<Local>,
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

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

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

    pub async fn create_bucket(
        &self,
        attr: StorageBucketAttr,
    ) -> AppResult<StorageBucketAttr, Errors> {
        let bucket_name = attr.name.clone();
        let mut storage = self.0.write().unwrap();
        if storage.contains_key(&bucket_name) {
            return Err(Errors::AlreadyExists {
                message: "Bucket already exists".into(),
            });
        }
        storage.insert(
            bucket_name.clone(),
            Arc::new(OnMemoryStorageBucket {
                attr,
                objects: BTreeMap::new(),
            }),
        );
        storage
            .get(&bucket_name)
            .map(|b| b.attr.clone())
            // Shouldn't happen but just in case
            .ok_or(Errors::FailedToWriteStorage {
                id: bucket_name,
                message: "Failed to create a new bucket".into(),
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        sync::{Arc, RwLock},
    };

    use googletest::prelude::*;

    use crate::{
        libs::errors::Errors,
        storage::{OnMemoryStorageBucket, Storage, StorageBucketAttr},
    };

    trait TestStorageExt {
        fn empty() -> Self;
    }

    impl TestStorageExt for Storage {
        fn empty() -> Self {
            Storage(Arc::new(RwLock::new(BTreeMap::new())))
        }
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_all_passed_buckets() {
        // Arrange
        let attr1 = StorageBucketAttr {
            name: "test_bucket_1".to_string(),
            versioning: false,
            default_event_based_hold: false,
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket1 = OnMemoryStorageBucket {
            attr: attr1.clone(),
            objects: BTreeMap::new(),
        };
        let attr2 = StorageBucketAttr {
            name: "test_bucket_2".to_string(),
            versioning: false,
            default_event_based_hold: false,
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
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

        // Act
        let res = storage.read_all().await;

        // Assert
        assert_eq!(res, vec![attr1, attr2]);
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_create_bucket_after_creating_new_bucket() {
        // Arrange
        let attr = StorageBucketAttr {
            name: "test_new_bucket".to_string(),
            versioning: true,
            default_event_based_hold: false,
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let storage = Storage::empty();

        // Act
        let res = storage.create_bucket(attr.clone()).await;

        // Assert
        assert_that!(res, ok(eq(attr)))
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_conflict_error_when_bucket_already_exists() {
        // Arrange
        let attr = StorageBucketAttr {
            name: "test_new_bucket".to_string(),
            versioning: true,
            default_event_based_hold: false,
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let storage = Storage::empty();
        let _ = storage.create_bucket(attr.clone()).await;

        // Act
        let res = storage.create_bucket(attr.clone()).await;

        // Assert
        expect_that!(res, err(matches_pattern!(Errors::AlreadyExists { .. })));
    }
}
