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
    pub fn add_document(
        &self, doc_id: &str, no_save: bool, score: f32,
        payload: &str, mut replace: bool, partial: bool,
        language: &str, fields: Vec<(&str, &str)>
    ) -> redis::RedisResult<()> {
        let mut cmd = redis::cmd("FT.ADD");
        cmd.arg(self.index_name.clone()).arg(doc_id).arg(score);

        if partial {
            replace = true;
        }
        if no_save {
            cmd.arg("NOSAVE");
        }
        if payload != "" {
            cmd.arg("PAYLOAD");
            cmd.arg(payload);
        }
        if replace {
            cmd.arg("REPLACE");
            if partial {
                cmd.arg("PARTIAL");
            }
        }
        if language != "" {
            cmd.arg("LANGUAGE");
            cmd.arg(language);
        }
        cmd.arg("FIELDS");
        // FIXME: Find better way
        let flatten_fields: Vec<String> = fields.iter()
            .map(|(f, v)| [f.to_string(), v.to_string()].join(" "))
            .collect();
        for field_value in flatten_fields.into_iter() {
            cmd.arg(field_value);
        }
        let _ : () = try!(cmd.query(&self.conn));
        Ok(())
    }

    /// Delete a document from index
    pub fn delete_document(&self, doc_id: &str) -> redis::RedisResult<()> {
        let mut cmd = redis::cmd("FT.DEL");
        cmd.arg(self.index_name.clone()).arg(doc_id);
        let _ : () = try!(cmd.query(&self.conn));
        Ok(())
    }

    fn _mk_query_args(&self) {
        unimplemented!()
    }

    /// Search the index for a given query, and return a result of documents
    pub fn search(&self) {
        unimplemented!()
    }
}
