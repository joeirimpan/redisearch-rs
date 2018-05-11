use redis;

use schema::Field;

pub struct RediSearch {
    index_name: String,
    conn: redis::Connection
}

impl RediSearch {

    pub fn new(index_name: &str, redis_url: &str) -> Self {
        let client = redis::Client::open(redis_url).unwrap();
        let connection = client.get_connection().unwrap();
        Self {
            index_name: index_name.to_string(),
            conn : connection
        }
    }

    /// Create the search index. The index must not already exist.
    pub fn create_index(&self, fields: Vec<Box<Field>>) -> redis::RedisResult<()> {
        let mut cmd = redis::cmd("FT.CREATE");
        cmd.arg(self.index_name.clone()).arg("SCHEMA");
        for field in fields.into_iter() {
            field.to_redis_args(&mut cmd);
        }
        let _ : () = try!(cmd.query(&self.conn));
        Ok(())
    }

    /// Drop the index if it exists
    pub fn drop_index(&self) -> redis::RedisResult<()> {
        let mut cmd = redis::cmd("FT.DROP");
        cmd.arg(self.index_name.clone());
        let _ : () = try!(cmd.query(&self.conn));
        Ok(())
    }

    /// Add a single document to the index.
    pub fn add_document(&self) {
        unimplemented!()
    }

    /// Delete a document from index
    pub fn delete_document(&self) {
        unimplemented!()
    }

    /// Load a single document by id
    pub fn load_document(&self) {
        unimplemented!()
    }

    /// Get info an stats about the the current index, including the number of documents, memory
    /// consumption, etc
    pub fn info(&self) {
        unimplemented!()
    }

    fn _mk_query_args(&self) {
        unimplemented!()
    }

    /// Search the index for a given query, and return a result of documents
    pub fn search(&self) {
        unimplemented!()
    }

    pub fn explain(&self) {
        unimplemented!()
    }
}
