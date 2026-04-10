//! RocksDB存储封装 - 链数据持久化、快速读写
use rocksdb::{DB, Options, WriteBatch};
use std::path::Path;

pub struct BlockchainDB {
    db: DB,
    path: String,
}

impl BlockchainDB {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_open_files(1024);
        opts.set_allow_mmap_writes(true);
        
        let db = DB::open(&opts, path).unwrap();
        
        Self {
            db,
            path: path.to_string(),
        }
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> bool {
        self.db.put(key, value).is_ok()
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).unwrap_or(None)
    }

    pub fn delete(&self, key: &[u8]) -> bool {
        self.db.delete(key).is_ok()
    }

    pub fn batch_write(&self, batch: WriteBatch) -> bool {
        self.db.write(batch).is_ok()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec<u8>, Vec<u8>)> {
        self.db.iterator(rocksdb::IteratorMode::Start)
    }

    pub fn close(&self) {
        let _ = DB::destroy(&Options::default(), Path::new(&self.path));
    }
}
