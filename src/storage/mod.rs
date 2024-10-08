use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local};
use dashmap::DashMap;

use crate::libs::errors::{AppResult, Errors};

#[derive(Debug, Clone, PartialEq)]
pub struct StorageBucketAttr {
    pub name: String,
    pub versioning: bool,
    pub default_event_based_hold: bool,
    pub location: String,
    pub time_created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateBucketAttr {
    pub versioning: bool,
    pub default_event_based_hold: bool,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateBucketAttr {
    pub versioning: Option<bool>,
    pub default_event_based_hold: bool,
}

#[derive(Clone, Debug)]
pub struct OnMemoryStorageBucket {
    pub attr: StorageBucketAttr,
    pub objects: DashMap<String, OnMemoryStorageObject>,
}

impl OnMemoryStorageBucket {
    pub fn replace_attr(&mut self, attr: StorageBucketAttr) {
        self.attr = attr;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OnMemoryStorageObject {
    // TODO: add attr
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub struct Storage(Arc<DashMap<String, Arc<Mutex<OnMemoryStorageBucket>>>>);
impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregates operations for a bucket.
pub trait BucketStorageExt {
    /// Corresponds to `list` operation: https://cloud.google.com/storage/docs/json_api/v1/buckets/list
    async fn list(&self) -> Vec<StorageBucketAttr>;

    /// Corresponds to `get` operation: https://cloud.google.com/storage/docs/json_api/v1/buckets/get
    async fn get(&self, name: &str) -> Option<StorageBucketAttr>;

    /// Corresponds to `insert` operation: https://cloud.google.com/storage/docs/json_api/v1/buckets/insert
    async fn create(
        &self,
        name: &str,
        attr: CreateBucketAttr,
    ) -> AppResult<StorageBucketAttr, Errors>;

    /// Corresponds to `patch` and `update` operation.
    /// Patch: https://cloud.google.com/storage/docs/json_api/v1/buckets/patch
    /// Update: https://cloud.google.com/storage/docs/json_api/v1/buckets/update
    async fn update(
        &self,
        name: &str,
        attr: UpdateBucketAttr,
    ) -> AppResult<StorageBucketAttr, Errors>;

    /// Corresponds to `delete` operation: https://cloud.google.com/storage/docs/json_api/v1/buckets/delete
    async fn delete(&self, name: &str) -> AppResult<StorageBucketAttr, Errors>;
}

impl BucketStorageExt for Storage {
    async fn list(&self) -> Vec<StorageBucketAttr> {
        let buckets = self
            .0
            .iter()
            .map(|b| {
                let bucket = b.value();
                bucket.lock().unwrap().clone()
            })
            .collect::<Vec<OnMemoryStorageBucket>>();
        buckets.iter().map(|b| b.attr.clone()).collect()
    }

    async fn get(&self, name: &str) -> Option<StorageBucketAttr> {
        self.0.get(name).map(|b| {
            let bucket = b.value();
            bucket.lock().unwrap().clone().attr
        })
    }

    async fn create(
        &self,
        name: &str,
        attr: CreateBucketAttr,
    ) -> AppResult<StorageBucketAttr, Errors> {
        if self.0.contains_key(name) {
            return Err(Errors::AlreadyExists {
                message: "Bucket already exists".into(),
            });
        }

        self.0.insert(
            name.to_string(),
            Arc::new(Mutex::new(OnMemoryStorageBucket {
                attr: StorageBucketAttr {
                    name: name.to_string(),
                    versioning: attr.versioning,
                    default_event_based_hold: attr.default_event_based_hold,
                    location: attr.location,
                    time_created: Local::now(),
                    updated: Local::now(),
                },
                objects: DashMap::new(),
            })),
        );

        self.0
            .get(name)
            .map(|b| {
                let bucket = b.value();
                bucket.lock().unwrap().clone().attr
            })
            .ok_or(Errors::FailedToWriteStorage {
                id: name.to_string(),
                message: "Failed to create a new bucket".into(),
            })
    }

    async fn update(
        &self,
        name: &str,
        attr: UpdateBucketAttr,
    ) -> AppResult<StorageBucketAttr, Errors> {
        let existence_bucket = self.0.get_mut(name).ok_or(Errors::BucketNotFound {
            message: "Bucket not found".into(),
        })?;

        let mut existence_bucket = existence_bucket.lock().unwrap();

        let new_attr = StorageBucketAttr {
            name: existence_bucket.attr.name.clone(),
            versioning: attr.versioning.unwrap_or(existence_bucket.attr.versioning),
            default_event_based_hold: attr.default_event_based_hold,
            location: existence_bucket.attr.location.clone(),
            time_created: existence_bucket.attr.time_created,
            updated: Local::now(),
        };
        existence_bucket.replace_attr(new_attr);
        Ok(existence_bucket.attr.clone())
    }

    async fn delete(&self, name: &str) -> AppResult<StorageBucketAttr, Errors> {
        self.0
            .remove(name)
            .ok_or(Errors::BucketNotFound {
                message: "Bucket not found".into(),
            })
            .map(|b| {
                let bucket = b.1.lock().unwrap();
                bucket.attr.clone()
            })
    }
}

impl Storage {
    pub fn new() -> Self {
        Storage(Arc::new(DashMap::new()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use dashmap::DashMap;
    use googletest::{assert_pred, prelude::*};

    use crate::{
        libs::errors::Errors,
        storage::{
            BucketStorageExt, CreateBucketAttr, OnMemoryStorageBucket, Storage, StorageBucketAttr,
        },
    };

    trait TestStorageExt {
        fn empty() -> Self;
    }

    impl TestStorageExt for Storage {
        fn empty() -> Self {
            Storage(Arc::new(DashMap::new()))
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
            location: "US-EAST1".into(),
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket1 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr1.clone(),
            objects: DashMap::new(),
        }));
        let attr2 = StorageBucketAttr {
            name: "test_bucket_2".to_string(),
            versioning: false,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket2 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr2.clone(),
            objects: DashMap::new(),
        }));

        let map = DashMap::new();
        map.insert("test_bucket_1".to_string(), bucket1);
        map.insert("test_bucket_2".to_string(), bucket2);

        let storage = Storage(Arc::new(map));

        // Act
        let mut res = storage.list().await;
        // To avoid flaky tests
        res.sort_by(|a, b| a.name.cmp(&b.name));

        // Assert
        assert_that!(res, eq(&vec![attr1, attr2]));
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_specific_bucket() {
        // Arrange
        let attr1 = StorageBucketAttr {
            name: "test_bucket_1".to_string(),
            versioning: false,
            default_event_based_hold: false,
            time_created: chrono::Local::now(),
            location: "US-EAST1".into(),
            updated: chrono::Local::now(),
        };
        let bucket1 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr1.clone(),
            objects: DashMap::new(),
        }));
        let attr2 = StorageBucketAttr {
            name: "test_bucket_2".to_string(),
            versioning: false,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket2 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr2.clone(),
            objects: DashMap::new(),
        }));

        let map = DashMap::new();
        map.insert("test_bucket_1".to_string(), bucket1);
        map.insert("test_bucket_2".to_string(), bucket2);

        let storage = Storage(Arc::new(map));

        // Act
        let res = storage.get("test_bucket_2").await;

        // Assert
        assert_that!(res, some(eq(&attr2)));
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_no_bucket_if_passed_non_exist_bucket_name() {
        // Arrange
        let attr1 = StorageBucketAttr {
            name: "test_bucket_1".to_string(),
            versioning: false,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket1 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr1.clone(),
            objects: DashMap::new(),
        }));
        let attr2 = StorageBucketAttr {
            name: "test_bucket_2".to_string(),
            versioning: false,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
            time_created: chrono::Local::now(),
            updated: chrono::Local::now(),
        };
        let bucket2 = Arc::new(Mutex::new(OnMemoryStorageBucket {
            attr: attr2.clone(),
            objects: DashMap::new(),
        }));

        let map = DashMap::new();
        map.insert("test_bucket_1".to_string(), bucket1);
        map.insert("test_bucket_2".to_string(), bucket2);

        let storage = Storage(Arc::new(map));

        // Act
        let res = storage.get("non-exist").await;

        // Assert
        assert_that!(res, none());
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_create_bucket_after_creating_new_bucket() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();

        // Act
        let res = storage.create("test_new_bucket", attr).await;

        // Assert
        assert_pred!(res.is_ok());
        let res = res.unwrap();
        expect_that!(res.name, eq("test_new_bucket"));
        expect_that!(res.versioning, eq(true));
        expect_that!(res.default_event_based_hold, eq(false));
        expect_that!(res.location, eq("US-EAST1"));
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_conflict_error_when_bucket_already_exists() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();
        let _ = storage.create("test_new_bucket", attr.clone()).await;

        // Act
        let res = storage.create("test_new_bucket", attr).await;

        // Assert
        assert_that!(res, err(matches_pattern!(Errors::AlreadyExists { .. })));
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_updated_bucket_after_updating_existence_bucket() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();
        let _ = storage.create("test_new_bucket", attr).await;

        // Act
        let res = storage
            .update(
                "test_new_bucket",
                crate::storage::UpdateBucketAttr {
                    versioning: Some(false),
                    default_event_based_hold: true,
                },
            )
            .await;

        // Assert
        assert_pred!(res.is_ok());
        let res = res.unwrap();
        expect_that!(res.versioning, eq(false));
        expect_that!(res.default_event_based_hold, eq(true));
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_not_found_error_while_updating_bucket() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();
        let _ = storage.create("test_new_bucket", attr).await;

        // Act
        let res = storage
            .update(
                "non_exist_bucket",
                crate::storage::UpdateBucketAttr {
                    versioning: Some(false),
                    default_event_based_hold: true,
                },
            )
            .await;

        // Assert
        assert_that!(res, err(matches_pattern!(Errors::BucketNotFound { .. })));
    }

    #[googletest::test]
    #[tokio::test]
    async fn can_delete_existing_bucket() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();
        let _ = storage.create("test_new_bucket", attr).await;

        // Act
        let res = storage.delete("test_new_bucket").await;
        let get_again = storage.get("test_new_bucket").await;

        // Assert
        assert_pred!(res.is_ok());
        assert_pred!(get_again.is_none());
    }

    #[googletest::test]
    #[tokio::test]
    async fn return_not_found_error_while_deleting_non_existing_bucket() {
        // Arrange
        let attr = CreateBucketAttr {
            versioning: true,
            default_event_based_hold: false,
            location: "US-EAST1".into(),
        };
        let storage = Storage::empty();
        let _ = storage.create("test_new_bucket", attr).await;

        // Act
        let res = storage.delete("non_exist_bucket").await;

        // Assert
        assert_that!(res, err(matches_pattern!(Errors::BucketNotFound { .. })));
    }
}
