use super::storage::RocksStorage;
use dozer_types::log::debug;
use rocksdb::WriteBatch;
use std::sync::Arc;
use std::time::Instant;

pub trait Writer<T> {
    fn begin(&mut self);
    fn insert(&mut self, key: &[u8], encoded: Vec<u8>);
    fn commit(&mut self, storage_client: &Arc<T>);
}

pub struct BatchedRocksDbWriter {
    batch: Option<WriteBatch>,
}

impl BatchedRocksDbWriter {
    pub fn new() -> Self {
        Self {
            batch: Option::from(WriteBatch::default()),
        }
    }
}

impl Writer<RocksStorage> for BatchedRocksDbWriter {
    fn begin(&mut self) {
        let _ = self.batch.insert(WriteBatch::default());
    }

    fn insert(&mut self, key: &[u8], encoded: Vec<u8>) {
        let _ = &self.batch.as_mut().unwrap().put(key, encoded);
    }

    fn commit(&mut self, storage_client: &Arc<RocksStorage>) {
        let before = Instant::now();
        let batch = self.batch.take().unwrap();
        storage_client
            .get_db()
            .write(batch)
            .expect("TODO: panic message");
        self.batch = Option::from(WriteBatch::default());
        debug!("Batch flush took: {:.2?}", before.elapsed(),);
    }
}