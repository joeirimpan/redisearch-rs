extern crate redis;

pub mod client;
pub mod schema;

use client::RediSearch;
use schema::{TextField, NumericField, TagField};

fn main() {
    let client = RediSearch::new("test_index", "redis://127.0.0.1:6379");
    let text_field = Box::new(
        TextField::new(
            "field1".to_string(),
            "TEXT".to_string(),
            false,
            100.0,
            false,
            false
        )
    );
    let num_field = Box::new(
        NumericField::new(
            "field2".to_string(),
            "NUMERIC".to_string(),
            false,
            false
        )
    );
    let tag_field = Box::new(
        TagField::new(
            "field3".to_string(),
            "TAG".to_string(),
            "-".to_string(),
            false,
            false,
        )
    );
    client.create_index(vec![text_field, num_field, tag_field]);
    client.drop_index();
}
