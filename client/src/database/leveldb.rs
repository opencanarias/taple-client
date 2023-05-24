use db_key;
use leveldb::options::Options as LevelDBOptions;
use leveldb::{
    database::Database,
    iterator::{Iterable, Iterator as LevelIterator, LevelDBIterator, RevIterator},
    kv::KV,
};
use std::cell::Cell;
use std::path::Path;
use std::sync::Arc;
use taple_core::{DatabaseCollection, DatabaseManager, DbError};

#[derive(Debug, PartialEq, Eq)]
pub struct StringKey(pub String);
impl db_key::Key for StringKey {
    fn from_u8(key: &[u8]) -> Self {
        Self(String::from_utf8(key.to_vec()).unwrap())
    }

    fn as_slice<T, F: Fn(&[u8]) -> T>(&self, f: F) -> T {
        let dst = self.0.as_bytes();
        f(&dst)
    }
}

#[derive(Clone, Copy)]
struct ReadOptions {
    fill_cache: bool,
    verify_checksums: bool,
}

impl<'a, K> From<ReadOptions> for leveldb::options::ReadOptions<'a, K>
where
    K: db_key::Key,
{
    fn from(item: ReadOptions) -> Self {
        let mut options = leveldb::options::ReadOptions::new();
        options.fill_cache = item.fill_cache;
        options.verify_checksums = item.verify_checksums;
        options
    }
}

fn get_initial_options() -> LevelDBOptions {
    let mut db_options = LevelDBOptions::new();
    db_options.create_if_missing = true;
    db_options
}

pub fn open_db(path: &Path) -> Arc<Database<StringKey>> {
    let db_options = get_initial_options();
    if let Ok(db) = Database::<StringKey>::open(path, db_options) {
        Arc::new(db)
    } else {
        panic!("Error opening DB with comparator")
    }
}

pub struct SyncCell<T>(Cell<T>);
unsafe impl<T> Sync for SyncCell<T> {}

pub struct LevelDBManager {
    db: Arc<Database<StringKey>>,
}

impl LevelDBManager {
    pub fn new(db: Arc<Database<StringKey>>) -> Self {
        Self { db }
    }
}

impl DatabaseManager<LDBCollection> for LevelDBManager {
    fn create_collection(&self, _identifier: &str) -> LDBCollection {
        LDBCollection {
            data: self.db.clone(),
            read_options: SyncCell(Cell::new(None)),
            write_options: SyncCell(Cell::new(None)),
        }
    }
}

pub struct LDBCollection {
    data: Arc<Database<StringKey>>,
    read_options: SyncCell<Option<ReadOptions>>,
    write_options: SyncCell<Option<leveldb::options::WriteOptions>>,
}

impl LDBCollection {
    fn generate_key(&self, key: &str) -> StringKey {
        StringKey(key.to_string())
    }

    pub fn get_read_options(&self) -> leveldb::options::ReadOptions<StringKey> {
        if let Some(options) = self.read_options.0.get() {
            return leveldb::options::ReadOptions::from(options);
        } else {
            return leveldb::options::ReadOptions::new();
        }
    }

    fn get_write_options(&self) -> leveldb::options::WriteOptions {
        if let Some(options) = self.write_options.0.get() {
            return options;
        } else {
            let mut write_options = leveldb::options::WriteOptions::new();
            write_options.sync = true;
            return write_options;
        }
    }
}

impl DatabaseCollection for LDBCollection {
    fn get(&self, key: &str) -> Result<Vec<u8>, DbError> {
        let key = self.generate_key(key);
        let result = self.data.get(self.get_read_options(), key);
        match result {
            Err(_) => Err(DbError::EntryNotFound),
            Ok(data) => match data {
                Some(value) => Ok(value),
                None => Err(DbError::EntryNotFound),
            },
        }
    }

    fn put(&self, key: &str, data: Vec<u8>) -> Result<(), DbError> {
        let key = self.generate_key(key);
        let _result = self.data.put(self.get_write_options(), key, &data);
        Ok(())
    }

    fn del(&self, key: &str) -> Result<(), DbError> {
        let key = self.generate_key(key);
        let _result = self.data.delete(self.get_write_options(), key);
        Ok(())
    }

    fn iter<'a>(
        &'a self,
        reverse: bool,
        prefix: String,
    ) -> Box<dyn Iterator<Item = (String, Vec<u8>)> + 'a> {
        if reverse {
            let iter = self.data.iter(self.get_read_options()).reverse();
            iter.seek(&StringKey(format!(
                "{}{}{}",
                prefix.clone(),
                char::MAX,
                char::MAX
            )));
            let mut alt_iter = iter.peekable();
            let iter = if let Some(_) = alt_iter.peek() {
                let mut iter = self.data.iter(self.get_read_options()).reverse();
                iter.seek(&StringKey(format!(
                    "{}{}{}",
                    prefix.clone(),
                    char::MAX,
                    char::MAX
                )));
                iter.advance();
                iter
            } else {
                self.data.iter(self.get_read_options()).reverse()
            };
            Box::new(RevLDBIterator::new(iter, prefix))
        } else {
            Box::new(LDBIterator::new(
                self.data.iter(self.get_read_options()),
                prefix,
            ))
        }
    }
}

pub struct LDBIterator<'a> {
    iter: LevelIterator<'a, StringKey>,
    table_name: String,
}

impl<'a> LDBIterator<'a> {
    pub fn new(iter: LevelIterator<'a, StringKey>, table_name: String) -> Self {
        iter.seek(&StringKey(table_name.clone()));
        Self { iter, table_name }
    }
}

impl<'a> Iterator for LDBIterator<'a> {
    type Item = (String, Vec<u8>);
    fn next(&mut self) -> Option<(String, Vec<u8>)> {
        loop {
            let item = self.iter.next();
            let Some(item) = item else {
                return None;
            };
            let key = {
                let StringKey(value) = item.0;
                if !value.starts_with(&self.table_name) {
                    return None;
                }
                value.replace(&self.table_name, "")
            };
            return Some((key, item.1));
        }
    }
}

pub struct RevLDBIterator<'a> {
    iter: RevIterator<'a, StringKey>,
    table_name: String,
}

impl<'a> RevLDBIterator<'a> {
    pub fn new(iter: RevIterator<'a, StringKey>, table_name: String) -> Self {
        Self { iter, table_name }
    }
}

impl<'a> Iterator for RevLDBIterator<'a> {
    type Item = (String, Vec<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next();
            let Some(item) = item else {
                return None;
            };
            let key = {
                let StringKey(value) = item.0;
                value.replace(&self.table_name, "")
            };
            return Some((key, item.1));
        }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use std::vec;
    use taple_core::DbError;
    use tempfile::tempdir;

    use crate::database::leveldb::open_db;

    use super::{DatabaseCollection, DatabaseManager, LDBCollection, LevelDBManager};

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    struct Data {
        id: usize,
        value: String,
    }

    fn get_data() -> Result<Vec<Vec<u8>>, DbError> {
        let data1 = Data {
            id: 1,
            value: "A".into(),
        };
        let data2 = Data {
            id: 2,
            value: "B".into(),
        };
        let data3 = Data {
            id: 3,
            value: "C".into(),
        };
        let Ok(data1) = bincode::serialize::<Data>(&data1) else {
            return Err(DbError::SerializeError);
        };
        let Ok(data2) = bincode::serialize::<Data>(&data2) else {
            return Err(DbError::SerializeError);
        };
        let Ok(data3) = bincode::serialize::<Data>(&data3) else {
            return Err(DbError::SerializeError);
        };
        Ok(vec![data1, data2, data3])
    }

    #[test]
    fn basic_operations_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDBManager::new(open_db(temp_dir.path()));
        let first_collection = db.create_collection("");
        let data = get_data().unwrap();
        // PUT & GET Operations
        // PUT
        let result = first_collection.put("a", data[0].clone());
        assert!(result.is_ok());
        let result = first_collection.put("b", data[1].clone());
        assert!(result.is_ok());
        let result = first_collection.put("c", data[2].clone());
        assert!(result.is_ok());
        // GET
        let result = first_collection.get("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data[0]);
        let result = first_collection.get("b");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data[1]);
        let result = first_collection.get("c");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data[2]);
        // DEL
        let result = first_collection.del("a");
        assert!(result.is_ok());
        let result = first_collection.del("b");
        assert!(result.is_ok());
        let result = first_collection.del("c");
        assert!(result.is_ok());
        // GET OF DELETED ENTRIES
        let result = first_collection.get("a");
        assert!(result.is_err());
        let result = first_collection.get("b");
        assert!(result.is_err());
        let result = first_collection.get("c");
        assert!(result.is_err());
    }

    #[test]
    fn partitions_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDBManager::new(open_db(temp_dir.path()));
        let first_collection = db.create_collection("");
        let second_collection = db.create_collection("");
        let data = get_data().unwrap();
        // PUT UNIQUE ENTRIES IN EACH PARTITION
        let result = first_collection.put("a", data[0].to_owned());
        assert!(result.is_ok());
        let result = second_collection.put("b", data[1].to_owned());
        assert!(result.is_ok());
        // NO EXIST IDIVIDUALITY
        let result = first_collection.get("b");
        assert_eq!(result.unwrap(), data[1]);
        let result = second_collection.get("a");
        assert_eq!(result.unwrap(), data[0]);
    }

    fn build_state(collection: &LDBCollection) {
        let data = get_data().unwrap();
        let result = collection.put("a", data[0].to_owned());
        assert!(result.is_ok());
        let result = collection.put("b", data[1].to_owned());
        assert!(result.is_ok());
        let result = collection.put("c", data[2].to_owned());
        assert!(result.is_ok());
    }

    fn build_initial_data() -> (Vec<&'static str>, Vec<Vec<u8>>) {
        let keys = vec!["a", "b", "c"];
        let data = get_data().unwrap();
        let values = vec![data[0].to_owned(), data[1].to_owned(), data[2].to_owned()];
        (keys, values)
    }

    #[test]
    fn iterator_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDBManager::new(open_db(temp_dir.path()));
        let first_collection = db.create_collection("");
        build_state(&first_collection);
        // ITER TEST
        let mut iter = first_collection.iter(false, "first".to_string());
        assert!(iter.next().is_none());
        let mut iter = first_collection.iter(false, "".to_string());
        let (keys, data) = build_initial_data();
        for i in 0..3 {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn rev_iterator_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDBManager::new(open_db(temp_dir.path()));
        let first_collection = db.create_collection("");
        build_state(&first_collection);
        // ITER TEST
        let mut iter = first_collection.iter(true, "first".to_string());
        assert!(iter.next().is_none());
        let mut iter = first_collection.iter(true, "".to_string());
        let (keys, data) = build_initial_data();
        for i in (0..3).rev() {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());
    }
}
