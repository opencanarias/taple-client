use super::wrapper_level::{StringKey, WrapperLevelDB};
use crate::database::error::WrapperLevelDBErrors;
use leveldb::options::Options as LevelDBOptions;
use leveldb::{
    comparator::OrdComparator,
    database::Database,
    iterator::{Iterable, Iterator as LevelIterator, LevelDBIterator, RevIterator},
    kv::KV,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{marker::PhantomData, path::Path};
use taple_core::{DatabaseCollection, DatabaseManager, DbError};

pub fn open_db_with_comparator(
    path: &Path,
) -> std::sync::Arc<leveldb::database::Database<StringKey>> {
    let mut db_options = LevelDBOptions::new();
    db_options.create_if_missing = true;
    let comparator = OrdComparator::<StringKey>::new("taple_comp".into());

    if let Ok(db) =
        crate::database::wrapper_level::open_db_with_comparator(path, db_options, comparator)
    {
        std::sync::Arc::new(db)
    } else {
        panic!("Error opening DB with comparator")
    }
}

pub struct LevelDB {
    db: std::sync::Arc<Database<StringKey>>,
}

impl DatabaseManager for LevelDB {
    fn create_collection<V>(
        &self,
        identifier: &str,
    ) -> Box<dyn DatabaseCollection<InnerDataType = V>>
    where
        V: Serialize + DeserializeOwned + Sync + Send + 'static,
    {
        Box::new(WrapperLevelDB::<StringKey, V>::new(
            self.db.clone(),
            identifier,
        ))
    }
}

impl LevelDB {
    pub fn new(db: std::sync::Arc<Database<StringKey>>) -> Self {
        let iter = db.iter(leveldb::options::ReadOptions::new());
        // db.put(
        //     leveldb::options::WriteOptions::new(),
        //     StringKey(format!("event{}", char::MAX)),
        //     &vec![10],
        // )
        // .unwrap();
        for i in iter {
            println!("{}", i.0 .0);
        }
        Self { db }
    }
}

impl<V: Serialize + DeserializeOwned + Sync + Send> DatabaseCollection
    for WrapperLevelDB<StringKey, V>
{
    type InnerDataType = V;

    fn put(&self, key: &str, data: Self::InnerDataType) -> Result<(), DbError> {
        let result = self.put(key, data);
        match result {
            Ok(_) => Ok(()),
            Err(WrapperLevelDBErrors::SerializeError) => Err(DbError::SerializeError),
            Err(WrapperLevelDBErrors::LevelDBError { source }) => {
                Err(DbError::CustomError(source.to_string()))
            }
            Err(_) => unreachable!(),
        }
    }

    fn get(&self, key: &str) -> Result<Self::InnerDataType, DbError> {
        let result = self.get(key);
        match result {
            Err(WrapperLevelDBErrors::DeserializeError) => Err(DbError::DeserializeError),
            Err(WrapperLevelDBErrors::EntryNotFoundError) => Err(DbError::EntryNotFound),
            Err(WrapperLevelDBErrors::LevelDBError { source }) => {
                Err(DbError::CustomError(source.to_string()))
            }
            Ok(data) => Ok(data),
            _ => unreachable!(),
        }
    }

    fn del(&self, key: &str) -> Result<(), DbError> {
        let result = self.del(key);
        if let Err(WrapperLevelDBErrors::LevelDBError { source }) = result {
            return Err(DbError::CustomError(source.to_string()));
        }
        Ok(())
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (String, Self::InnerDataType)> + 'a> {
        let iter = self.db.iter(self.get_read_options());
        log::error!("TABLE NAME {}", self.get_table_name());
        Box::new(DbIterator::new(iter, self.get_table_name()))
    }

    fn rev_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (String, Self::InnerDataType)> + 'a> {
        let iter = self.db.iter(self.get_read_options()).reverse();
        iter.seek(&StringKey(self.create_last_key()));
        let mut alt_iter = iter.peekable();
        let iter = if let Some(_) = alt_iter.peek() {
            let mut iter = self.db.iter(self.get_read_options()).reverse();
            iter.seek(&StringKey(self.create_last_key()));
            iter.advance();
            iter
        } else {
            self.db.iter(self.get_read_options()).reverse()
        };
        Box::new(RevDbIterator::new(iter, self.get_table_name()))
    }

    fn partition<'a>(
        &'a self,
        key: &str,
    ) -> Box<dyn DatabaseCollection<InnerDataType = Self::InnerDataType> + 'a> {
        Box::new(self.partition(&key))
    }
}

pub struct DbIterator<'a, V: Serialize + DeserializeOwned> {
    _tmp: PhantomData<V>,
    table_name: String,
    iter: LevelIterator<'a, StringKey>,
}

impl<'a, V: Serialize + DeserializeOwned> DbIterator<'a, V> {
    pub fn new(iter: LevelIterator<'a, StringKey>, table_name: String) -> Self {
        iter.seek(&StringKey(table_name.clone()));
        Self {
            _tmp: PhantomData::default(),
            table_name,
            iter,
        }
    }
}

impl<'a, V: Serialize + DeserializeOwned> Iterator for DbIterator<'a, V> {
    type Item = (String, V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next();
            let Some(item) = item else {
              return None;
          };
            if !item.0 .0.starts_with(&self.table_name) {
                log::error!("OCURRE ESTO");
                log::error!("TABLE NAME ES {}", self.table_name);
                log::error!("{}", item.0.0);
                return None;
            }
            let value = WrapperLevelDB::<StringKey, V>::deserialize(item.1).unwrap();
            let key = {
                let StringKey(value) = item.0;
                value.replace(&self.table_name, "")
            };
            log::error!("DEVUELVE KEY");
            return Some((key, value));
        }
    }
}

pub struct RevDbIterator<'a, V: Serialize + DeserializeOwned + 'a> {
    _tmp: PhantomData<V>,
    table_name: String,
    iter: RevIterator<'a, StringKey>,
}

impl<'a, V: Serialize + DeserializeOwned> RevDbIterator<'a, V> {
    pub fn new(iter: RevIterator<'a, StringKey>, table_name: String) -> Self {
        Self {
            _tmp: PhantomData::default(),
            table_name,
            iter,
        }
    }
}

impl<'a, V: Serialize + DeserializeOwned> Iterator for RevDbIterator<'a, V> {
    type Item = (String, V);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next();
            let Some(item) = item else {
              return None;
          };
            if !item.0 .0.starts_with(&self.table_name) {
                return None;
            }
            let value = WrapperLevelDB::<StringKey, V>::deserialize(item.1).unwrap();
            let key = {
                let StringKey(value) = item.0;
                value.replace(&self.table_name, "")
            };
            return Some((key, value));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::database::wrapper_level::StringKey;
    use leveldb::comparator::OrdComparator;
    use leveldb::options::Options as LevelDBOptions;
    use serde::{Deserialize, Serialize};
    use std::{path::Path, vec};
    use tempfile::tempdir;

    use super::{DatabaseCollection, DatabaseManager, LevelDB};

    pub fn open_db(path: &Path) -> std::sync::Arc<leveldb::database::Database<StringKey>> {
        let mut db_options = LevelDBOptions::new();
        db_options.create_if_missing = true;

        if let Ok(db) = crate::database::wrapper_level::open_db(path, db_options) {
            std::sync::Arc::new(db)
        } else {
            panic!("Error opening DB")
        }
    }

    pub fn open_db_with_comparator(
        path: &Path,
    ) -> std::sync::Arc<leveldb::database::Database<StringKey>> {
        let mut db_options = LevelDBOptions::new();
        db_options.create_if_missing = true;
        let comparator = OrdComparator::<StringKey>::new("taple_comparator".into());

        if let Ok(db) =
            crate::database::wrapper_level::open_db_with_comparator(path, db_options, comparator)
        {
            std::sync::Arc::new(db)
        } else {
            panic!("Error opening DB with comparator")
        }
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    struct Data {
        id: usize,
        value: String,
    }

    #[test]
    fn basic_operations_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        // PUT & GET Operations
        // PUT
        let result = first_collection.put(
            "a",
            Data {
                id: 1,
                value: "A".into(),
            },
        );
        assert!(result.is_ok());
        let result = first_collection.put(
            "b",
            Data {
                id: 2,
                value: "B".into(),
            },
        );
        assert!(result.is_ok());
        let result = first_collection.put(
            "c",
            Data {
                id: 3,
                value: "C".into(),
            },
        );
        assert!(result.is_ok());
        // GET
        let result = first_collection.get("a");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Data {
                id: 1,
                value: "A".into()
            }
        );
        let result = first_collection.get("b");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Data {
                id: 2,
                value: "B".into()
            }
        );
        let result = first_collection.get("c");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Data {
                id: 3,
                value: "C".into()
            }
        );
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
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let second_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("second");
        // PUT UNIQUE ENTRIES IN EACH PARTITION
        let result = first_collection.put(
            "a",
            Data {
                id: 1,
                value: "A".into(),
            },
        );
        assert!(result.is_ok());
        let result = second_collection.put(
            "b",
            Data {
                id: 2,
                value: "B".into(),
            },
        );
        assert!(result.is_ok());
        // TRYING TO GET ENTRIES FROM A DIFFERENT PARTITION
        let result = first_collection.get("b");
        assert!(result.is_err());
        let result = second_collection.get("a");
        assert!(result.is_err());
    }

    #[test]
    fn inner_partition() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let inner_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            first_collection.partition("inner");
        // PUT OPERATIONS
        let result = first_collection.put(
            "a",
            Data {
                id: 1,
                value: "A".into(),
            },
        );
        assert!(result.is_ok());
        let result = inner_collection.put(
            "b",
            Data {
                id: 2,
                value: "B".into(),
            },
        );
        assert!(result.is_ok());
        // TRYING TO GET ENTRIES FROM A DIFFERENT PARTITION
        let result = first_collection.get("b");
        assert!(result.is_err());
        let result = inner_collection.get("a");
        assert!(result.is_err());
    }

    fn build_state(collection: &Box<dyn DatabaseCollection<InnerDataType = Data>>) {
        let result = collection.put(
            "a",
            Data {
                id: 1,
                value: "A".into(),
            },
        );
        assert!(result.is_ok());
        let result = collection.put(
            "b",
            Data {
                id: 2,
                value: "B".into(),
            },
        );
        assert!(result.is_ok());
        let result = collection.put(
            "c",
            Data {
                id: 3,
                value: "C".into(),
            },
        );
        assert!(result.is_ok());
    }

    fn build_initial_data() -> (Vec<&'static str>, Vec<Data>) {
        let keys = vec!["a", "b", "c"];
        let data = vec![
            Data {
                id: 1,
                value: "A".into(),
            },
            Data {
                id: 2,
                value: "B".into(),
            },
            Data {
                id: 3,
                value: "C".into(),
            },
        ];
        (keys, data)
    }

    #[test]
    fn iterator_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        build_state(&first_collection);
        // ITER TEST
        let mut iter = first_collection.iter();
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
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        build_state(&first_collection);
        // ITER TEST
        let mut iter = first_collection.rev_iter();
        let (keys, data) = build_initial_data();
        for i in (0..3).rev() {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn iterator_with_various_collection_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let second_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("second");
        build_state(&first_collection);
        let result = second_collection.put(
            "d",
            Data {
                id: 4,
                value: "D".into(),
            },
        );
        assert!(result.is_ok());
        let result = second_collection.put(
            "e",
            Data {
                id: 5,
                value: "E".into(),
            },
        );
        assert!(result.is_ok());
        let mut iter = first_collection.iter();
        let (keys, data) = build_initial_data();
        for i in 0..3 {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());

        let mut iter = second_collection.iter();
        let (keys, data) = (
            vec!["d", "e"],
            vec![
                Data {
                    id: 4,
                    value: "D".into(),
                },
                Data {
                    id: 5,
                    value: "E".into(),
                },
            ],
        );
        for i in 0..2 {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn rev_iterator_with_various_collection_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let second_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("second");
        build_state(&first_collection);
        let result = second_collection.put(
            "d",
            Data {
                id: 4,
                value: "D".into(),
            },
        );
        assert!(result.is_ok());
        let result = second_collection.put(
            "e",
            Data {
                id: 5,
                value: "E".into(),
            },
        );
        assert!(result.is_ok());
        let mut iter = first_collection.rev_iter();
        let (keys, data) = build_initial_data();
        for i in (0..3).rev() {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());

        let mut iter = second_collection.rev_iter();
        let (keys, data) = (
            vec!["d", "e"],
            vec![
                Data {
                    id: 4,
                    value: "D".into(),
                },
                Data {
                    id: 5,
                    value: "E".into(),
                },
            ],
        );
        for i in (0..2).rev() {
            let (key, val) = iter.next().unwrap();
            assert_eq!(keys[i], key);
            assert_eq!(data[i], val);
        }
        assert!(iter.next().is_none());
    }

    #[test]
    fn iteration_with_partitions_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let first_inner = first_collection.partition("inner1");
        let second_inner = first_inner.partition("inner2");
        first_collection
            .put(
                "a",
                Data {
                    id: 0,
                    value: "A".into(),
                },
            )
            .unwrap();
        first_inner
            .put(
                "b",
                Data {
                    id: 0,
                    value: "B".into(),
                },
            )
            .unwrap();
        second_inner
            .put(
                "c",
                Data {
                    id: 0,
                    value: "C".into(),
                },
            )
            .unwrap();
        let mut iter = second_inner.iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "c");
        assert!(iter.next().is_none());

        let mut iter = first_inner.iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "b");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner2\u{10ffff}c");
        assert!(iter.next().is_none());

        let mut iter = first_collection.iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "a");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner1\u{10ffff}b");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner1\u{10ffff}inner2\u{10ffff}c");
        assert!(iter.next().is_none());
    }

    #[test]
    fn rev_iteration_with_partitions_test() {
        let temp_dir = tempdir().unwrap();
        let db = LevelDB::new(open_db_with_comparator(temp_dir.path()));
        let first_collection: Box<dyn DatabaseCollection<InnerDataType = Data>> =
            db.create_collection("first");
        let first_inner = first_collection.partition("inner1");
        let second_inner = first_inner.partition("inner2");
        first_collection
            .put(
                "a",
                Data {
                    id: 0,
                    value: "A".into(),
                },
            )
            .unwrap();
        first_inner
            .put(
                "b",
                Data {
                    id: 0,
                    value: "B".into(),
                },
            )
            .unwrap();
        second_inner
            .put(
                "c",
                Data {
                    id: 0,
                    value: "C".into(),
                },
            )
            .unwrap();
        let mut iter = second_inner.rev_iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "c");
        assert!(iter.next().is_none());

        let mut iter = first_inner.rev_iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner2\u{10ffff}c");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "b");
        assert!(iter.next().is_none());

        let mut iter = first_collection.rev_iter();
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner1\u{10ffff}inner2\u{10ffff}c");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "inner1\u{10ffff}b");
        let item = iter.next().unwrap();
        assert_eq!(&item.0, "a");
        assert!(iter.next().is_none());
    }
}
