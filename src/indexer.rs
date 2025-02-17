use crate::log_loader;
use regex::Regex;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use chrono::{NaiveDateTime, Utc};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, Value, INDEXED, STORED, TEXT};
use tantivy::{
    DateOptions, DateTime, DocAddress, Document, Index, IndexWriter, Order, ReloadPolicy,
    TantivyDocument,
};

pub struct Indexer {
    schema: Schema,
}

impl Indexer {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("_raw", TEXT | STORED);

        let opts = DateOptions::from(INDEXED)
            .set_stored()
            .set_fast()
            .set_precision(tantivy::schema::DateTimePrecision::Seconds);
        schema_builder.add_date_field("_time", opts);

        // user defined patterns
        schema_builder.add_text_field("requestId", TEXT | STORED);

        let schema = schema_builder.build();



        Self {
            schema,
        }
    }

    pub fn index_logfile(&mut self) {
        let index = Index::create_in_dir("./data/indexes", self.schema.clone()).unwrap();
        let mut index_writer: IndexWriter = index.writer(50_000_000).unwrap();

        let lines = log_loader::from_logfile(String::from("./data/test.log"));

        // Regex to match key=value pairs
        let key_value_pattern = Regex::new(r"(\w+)=([^,]+)").unwrap();
        // HashMap to store extracted fields
        let mut log_data: HashMap<String, String> = HashMap::new();

        for (i, line) in lines.enumerate() {
            // Extract timestamp manually (always at the beginning)
            if let Some((_time, rest)) = line.split_once(',') {
                let _time = convert_to_iso8601(_time);
                // TODO: performance implication
                log_data.insert("_time".to_string(), _time);

                // Extract key-value pairs
                for cap in key_value_pattern.captures_iter(rest) {
                    let key = cap.get(1).unwrap().as_str().to_string();
                    let value = cap.get(2).unwrap().as_str().to_string();
                    log_data.insert(key, value);
                }
            }

            let mut old_man_doc = TantivyDocument::default();
            let ts = log_data.remove("_time").unwrap();
            let ts = self
                .schema
                .get_field_entry(self.field("_time"))
                .field_type()
                .value_from_json(JsonValue::String(ts))
                .unwrap();
            old_man_doc.add_field_value(self.field("_time"), ts);
            old_man_doc.add_text(self.field("requestId"), log_data.get("requestId").unwrap());
            old_man_doc.add_text(self.field("_raw"), line);
            index_writer.add_document(old_man_doc);
        }

        index_writer.commit().unwrap();
    }

    pub fn query(&mut self, query: String) -> Vec<HashMap<String, String>> {
        let index = Index::open_in_dir("./data/indexes").unwrap();

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .unwrap();

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![self.field("_raw")]);
        let query = query_parser.parse_query(&query).unwrap();

        // Sort by timestamp (ascending order)
        let top_docs: Vec<(DateTime, DocAddress)> = searcher
            .search(
                &query,
                &TopDocs::with_limit(1_000).order_by_fast_field("_time", Order::Asc),
            )
            .unwrap();

        top_docs
            .into_iter()
            .map(move |(_, doc_address)| {
                let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();

                println!("{}", retrieved_doc.to_json(&self.schema));

                let mut record = HashMap::default();
                for fv in retrieved_doc {
                    if self.schema.get_field_name(fv.field) == "_time" {
                        let _time = fv
                            .value
                            .as_ref()
                            .as_datetime()
                            .unwrap()
                            .into_timestamp_millis();
                        record.insert("_time".to_string(), _time.to_string());
                    } else {
                        record.insert(
                            self.schema.get_field_name(fv.field).to_string(),
                            fv.value.as_ref().as_str().unwrap().to_string(),
                        );
                    }
                }
                record
            })
            .collect::<Vec<HashMap<String, String>>>()
    }


    fn field(&self, field: &str) -> Field {
        self.schema.get_field(field).unwrap()
    }
}

fn convert_to_iso8601(input: &str) -> String {
    // Parse the input timestamp as NaiveDateTime (without timezone)
    let naive_datetime = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%.3f").unwrap();

    // Convert to a DateTime with UTC timezone
    let utc_datetime: chrono::DateTime<Utc> = chrono::DateTime::from_utc(naive_datetime, Utc);

    // Format the DateTime in the ISO 8601 format with a 'Z' at the end for UTC
    utc_datetime.to_rfc3339()
}