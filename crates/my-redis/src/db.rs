use bytes::Bytes;
use std::{
    collections::HashMap,
    hash::{BuildHasher, RandomState},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Db {
    inner: Arc<Vec<Mutex<ShardedDbInner>>>,
    hasher: RandomState,
}

struct ShardedDbInner {
    data: HashMap<String, Bytes>,
}

impl Db {
    #[must_use]
    pub fn new(num_shards: usize) -> Self {
        debug_assert!(num_shards > 0, "number of shards should be greater than 0");
        let mut db = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            db.push(Mutex::new(ShardedDbInner {
                data: HashMap::new(),
            }));
        }
        Self {
            inner: Arc::new(db),
            hasher: RandomState::default(),
        }
    }

    /// # Panics
    /// Panics if cannot acquire lock
    pub fn insert(&self, key: &str, value: Bytes) {
        #[allow(clippy::indexing_slicing)]
        let shard = &self.inner[self.get_shard_idx(key)];
        #[allow(clippy::unwrap_used)]
        let mut lock = shard.lock().unwrap();
        lock.data.insert(key.to_string(), value);
    }

    /// # Panics
    /// Panics if cannot acquire lock
    #[must_use]
    pub fn get(&self, key: &str) -> Option<Bytes> {
        #[allow(clippy::indexing_slicing)]
        let shard = &self.inner[self.get_shard_idx(key)];
        #[allow(clippy::unwrap_used)]
        let lock = shard.lock().unwrap();
        lock.data.get(key).cloned()
    }

    #[allow(
        clippy::as_conversions,
        clippy::arithmetic_side_effects,
        clippy::cast_possible_truncation
    )]
    fn get_shard_idx(&self, key: &str) -> usize {
        let key_hash = self.hasher.hash_one(key);
        let len = self.inner.len();

        ((u128::from(key_hash) * len as u128) >> 64) as usize
    }
}
