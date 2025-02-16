use std::any::Any;
use std::collections::HashMap;
use crate::log_loader;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, FieldType, Schema, SchemaBuilder, Value, FAST, INDEXED, STORED, TEXT};
use tantivy::{DateOptions, DateTime, DocAddress, Document, Index, IndexReader, IndexWriter, Order, ReloadPolicy, Searcher, TantivyDocument};
use tantivy::collector::TopDocs;
use regex::Regex;
use serde_json::Value as JsonValue;

pub struct Indexer {
    schema: Schema,
    raw: Field,
    timestamp: Field,
    request_id: Field,
}

impl Indexer {
    pub fn new() -> Self {

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("raw", TEXT | STORED);
        schema_builder.add_text_field("requestId", TEXT | STORED);
        let opts = DateOptions::from(INDEXED)
            .set_stored()
            .set_fast()
            .set_precision(tantivy::schema::DateTimePrecision::Seconds);
        schema_builder.add_date_field("timestamp",opts);
        let schema = schema_builder.build();


        let raw = schema.get_field("raw").unwrap();
        let timestamp = schema.get_field("timestamp").unwrap();
        let request_id = schema.get_field("requestId").unwrap();

        Self {
            schema,
            raw,
            timestamp,
            request_id
            
        }
    }

    pub fn index_logfile(&mut self) {

        let index = Index::create_in_dir("./data/indexes", self.schema.clone()).unwrap();
        let mut index_writer: IndexWriter = index.writer(50_000_000).unwrap();

        let lines = log_loader::from_logfile(String::from("./data/test.log"));

        // Regex to match key=value pairs
        let key_value_re = Regex::new(r"(\w+)=([^,]+)").unwrap();
        // HashMap to store extracted fields
        let mut log_data: HashMap<String, String> = HashMap::new();



        for (i, line) in lines.enumerate() {


            // Extract timestamp manually (always at the beginning)
            if let Some((timestamp, rest)) = line.split_once(' ') {
                log_data.insert("timestamp".to_string(), timestamp.to_string());

                // Extract key-value pairs
                for cap in key_value_re.captures_iter(rest) {
                    let key = cap.get(1).unwrap().as_str().to_string();
                    let value = cap.get(2).unwrap().as_str().to_string();
                    log_data.insert(key, value);
                }
            }
            
            
            let mut old_man_doc = TantivyDocument::default();
            let ts = log_data.get("timestamp").unwrap().clone();
            let ts = self.schema.get_field_entry(self.timestamp).field_type().value_from_json(JsonValue::String(ts)).unwrap();
            old_man_doc.add_field_value(self.timestamp, ts);
            old_man_doc.add_text(self.request_id, log_data.get("requestId").unwrap());
            old_man_doc.add_text(self.raw, line);
            index_writer.add_document(old_man_doc);
        }
            
        index_writer.commit().unwrap();


    }

    pub fn query(&mut self, query: String) -> Vec<HashMap<String,String>>  {

        let index = Index::open_in_dir("./data/indexes").unwrap();

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into().unwrap();

        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![self.raw]);
        let query = query_parser.parse_query(&query).unwrap();

        // Sort by timestamp (ascending order)
        let top_docs: Vec<(DateTime, DocAddress)> = searcher
            .search(&query, &TopDocs::with_limit(1_000).order_by_fast_field("timestamp", Order::Asc))
            .unwrap();


        top_docs.into_iter().map(move |(_, doc_address)| {

            let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
            
           println!("{}", retrieved_doc.to_json(&self.schema));
            
            let mut record = HashMap::default();
            for fv in retrieved_doc {
                if self.schema.get_field_name(fv.field) == "timestamp" {
                   let _timestamp = fv.value.as_ref().as_datetime().unwrap().into_timestamp_millis();
                    record.insert("_timestamp".to_string(), _timestamp.to_string());
                } else {
                    record.insert(self.schema.get_field_name(fv.field).to_string(), fv.value.as_ref().as_str().unwrap().to_string());
                }
            }
            record
            }).collect::<Vec<HashMap<String,String>>>()
    }
}
