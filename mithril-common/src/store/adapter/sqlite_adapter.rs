use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use rusqlite::{types::FromSql, Connection, Row, Statement, ToSql};
use tokio::sync::Mutex;

use super::{AdapterError, StoreAdapter};

type Result<T> = std::result::Result<T, AdapterError>;

/// Store adapter for SQLite3
struct SQLiteAdapter<'a, K, V> {
    structure: Box<dyn SqlTableDescription + Sync + Send>,
    connection: Arc<Mutex<Connection>>,
    key: &'a PhantomData<K>,
    value: &'a PhantomData<V>,
}

impl<'a, K, V> SQLiteAdapter<'a, K, V> {
    /// Create a new SQLiteAdapter instance.
    pub fn new(
        file: PathBuf,
        structure: Box<dyn SqlTableDescription + Sync + Send>,
    ) -> Result<Self> {
        let connection =
            Connection::open(file).map_err(|e| AdapterError::InitializationError(e.into()))?;

        Ok(Self {
            structure,
            connection: Arc::new(Mutex::new(connection)),
            key: &PhantomData,
            value: &PhantomData,
        })
    }
}

#[async_trait]
impl<K, V, 'a> StoreAdapter<'a> for SQLiteAdapter<'a, K, V>
where
    K: Send + Sync + ToSql + FromSql,
    V: Send + Sync + ToSql + FromSql + Clone,
{
    type Key = K;
    type Record = V;

    async fn store_record(&mut self, key: &Self::Key, record: &Self::Record) -> Result<()> {
        let connection = self.connection.lock().await;
        let sql = format!(
            "insert into {} ({}, {}) values (?1, ?2)",
            self.structure.get_table_name(),
            self.structure.get_key_field(),
            self.structure.get_record_field()
        );
        connection
            .execute(&sql, (key, record))
            .map(|_| ())
            .map_err(|e| AdapterError::MutationError(e.into()))
    }

    async fn get_record(&self, key: &Self::Key) -> Result<Option<Self::Record>> {
        let connection = self.connection.lock().await;
        let sql = format!(
            "select {} from {} where {} = ?1",
            self.structure.get_record_field(),
            self.structure.get_table_name(),
            self.structure.get_key_field()
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
        let mut res = statement
            .query([key])
            .map_err(|e| AdapterError::QueryError(e.into()))?;

        res.next()
            .map(|option| {
                option
                    .map(|row| row.get::<usize, V>(0).iter().next().cloned())
                    .flatten()
            })
            .map_err(|e| AdapterError::QueryError(e.into()))
    }

    async fn record_exists(&self, key: &Self::Key) -> Result<bool> {
        let connection = self.connection.lock().await;
        let sql = format!(
            "select exists(select 1 from {} where {} = ?1)",
            self.structure.get_table_name(),
            self.structure.get_key_field()
        );
        connection
            .query_row(
                &sql,
                [key],
                |row| Ok(row.get::<usize, u64>(0).unwrap() == 1),
            )
            .map_err(|e| AdapterError::QueryError(e.into()))
    }

    async fn get_last_n_records(&self, how_many: usize) -> Result<Vec<(Self::Key, Self::Record)>> {
        todo!()
    }

    async fn remove(&mut self, key: &Self::Key) -> Result<Option<Self::Record>> {
        todo!()
    }

    async fn get_iter<'iter: 'a>(&self) -> Result<Box<dyn Iterator<Item = Self::Record> + 'a>> {
        let connection = self.connection.lock().await;
        let sql = format!(
            "select {} from {} order by {} desc",
            self.structure.get_record_field(),
            self.structure.get_table_name(),
            self.structure.get_created_at_field()
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
        let map = statement
            .query_map([], Box::new(|row: &Row| row.get(0)))
            .map_err(|e| AdapterError::QueryError(e.into()))?
            .map(|row| row.iter().next().cloned().unwrap());
        let iterator: SQLiteResultIterator<'iter, Self::Record> =
            SQLiteResultIterator::spawn(Box::new(map), statement);

        Ok(Box::new(iterator))
    }
}

struct SQLiteResultIterator<'a, V> {
    iterator: Box<dyn Iterator<Item = V> + 'a>,
    statement: Statement<'a>,
}

impl<'a, V> SQLiteResultIterator<'a, V>
where
    V: FromSql + Clone,
{
    pub fn spawn(iterator: Box<dyn Iterator<Item = V> + 'a>, statement: Statement<'a>) -> Self {
        Self {
            iterator,
            statement,
        }
    }
}

impl<'a, V> Iterator for SQLiteResultIterator<'a, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

/// SqlProjection allow structures to be stored and fetched from a SQL database.
trait SqlTableDescription {
    /// Return the table name for queries.
    fn get_table_name(&self) -> &String;

    /// Return the field name of the Key.
    fn get_key_field(&self) -> &String;

    /// Return the field name of the Record.
    fn get_record_field(&self) -> &String;

    /// Return the field name where the timestamp of record creation is.
    fn get_created_at_field(&self) -> &String;
}

#[cfg(test)]
mod tests {

    use std::{
        borrow::Borrow,
        fs::{create_dir_all, remove_file},
    };

    use super::*;

    struct TestSqlStructure {
        table_name: String,
        key_field: String,
        record_field: String,
        created_at_field: String,
    }

    impl TestSqlStructure {
        pub fn new() -> Self {
            Self {
                table_name: "test_adapter".to_string(),
                key_field: "row_id".to_string(),
                record_field: "row_data".to_string(),
                created_at_field: "created_at".to_string(),
            }
        }
    }

    impl SqlTableDescription for TestSqlStructure {
        fn get_table_name(&self) -> &String {
            &self.table_name
        }

        fn get_key_field(&self) -> &String {
            &self.key_field
        }

        fn get_record_field(&self) -> &String {
            &self.record_field
        }

        fn get_created_at_field(&self) -> &String {
            &self.created_at_field
        }
    }

    fn get_file_path(test_name: &str) -> PathBuf {
        let dirpath = std::env::temp_dir().join("mithril_test");

        if !dirpath.exists() {
            create_dir_all(&dirpath).expect(&format!(
                "Expecting to be able to create the test directory '{}'.",
                dirpath.to_string_lossy()
            ));
        }

        dirpath.join(format!("{}.sqlite3", test_name))
    }

    fn init_db(test_name: &str) -> SQLiteAdapter<u64, String> {
        let filepath = get_file_path(test_name);

        if filepath.exists() {
            remove_file(&filepath).expect(&format!(
                "Expecting to be able to remove the database file '{}'.",
                filepath.to_string_lossy()
            ));
        }
        let connection = Connection::open(&filepath).expect(&format!(
            "Expecting to be able to open SQLite file '{}'.",
            filepath.to_string_lossy()
        ));
        connection
            .execute(
                "create table test_adapter (row_id integer primary key, row_data text not null, created_at text default CURRENT_TIMESTAMP)",
                (),
            )
            .unwrap();

        let adapter = SQLiteAdapter::new(filepath, Box::new(TestSqlStructure::new())).unwrap();

        adapter
    }
    #[tokio::test]
    async fn test_store_record() {
        let test_name = "test_store_record";
        let mut adapter = init_db(test_name);
        adapter
            .store_record(&1, "one".to_string().borrow())
            .await
            .unwrap();
        let filepath = get_file_path(test_name);
        let connection = Connection::open(&filepath).expect(&format!(
            "Expecting to be able to open SQLite file '{}'.",
            filepath.to_string_lossy()
        ));
        let mut stmt = connection.prepare("select * from test_adapter").unwrap();
        let result = stmt
            .query_row([], |row| {
                Ok((
                    row.get::<usize, u64>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                ))
            })
            .unwrap();

        assert_eq!((1, "one".to_string()), result);
    }

    #[tokio::test]
    async fn test_get_record() {
        let test_name = "test_get_record";
        let mut adapter = init_db(test_name);
        adapter
            .store_record(&1, "one".to_string().borrow())
            .await
            .unwrap();
        adapter
            .store_record(&2, "two".to_string().borrow())
            .await
            .unwrap();
        adapter
            .store_record(&3, "three".to_string().borrow())
            .await
            .unwrap();
        assert_eq!(
            Some("one".to_string()),
            adapter.get_record(&1).await.unwrap()
        );
        assert_eq!(
            Some("three".to_string()),
            adapter.get_record(&3).await.unwrap()
        );
        assert_eq!(
            Some("two".to_string()),
            adapter.get_record(&2).await.unwrap()
        );
        assert_eq!(None, adapter.get_record(&4).await.unwrap());
    }

    #[tokio::test]
    async fn test_record_exists() {
        let test_name = "test_record_exists";
        let mut adapter = init_db(test_name);
        adapter
            .store_record(&1, "one".to_string().borrow())
            .await
            .unwrap();
        adapter
            .store_record(&2, "two".to_string().borrow())
            .await
            .unwrap();
        assert!(adapter.record_exists(&1).await.unwrap());
        assert!(adapter.record_exists(&2).await.unwrap());
        assert!(!adapter.record_exists(&3).await.unwrap());
    }

    #[tokio::test]
    async fn test_get_iter() {
        let test_name = "test_get_iter";
        let mut adapter = init_db(test_name);
        adapter
            .store_record(&1, "one".to_string().borrow())
            .await
            .unwrap();
        adapter
            .store_record(&2, "two".to_string().borrow())
            .await
            .unwrap();
        let iterator = adapter.get_iter().await.unwrap();

        for (idx, record) in iterator.enumerate() {
            match idx {
                0 => assert_eq!("one".to_string(), record),
                1 => assert_eq!("two".to_string(), record),
                v => panic!("unexpected result indice {}", v),
            }
        }
    }
}
