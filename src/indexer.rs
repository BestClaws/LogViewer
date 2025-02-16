use std::collections::HashMap;
use crate::log_loader;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, SchemaBuilder, Value, STORED, TEXT};
use tantivy::{Document, Index, IndexReader, IndexWriter, ReloadPolicy, Searcher, TantivyDocument};
use tantivy::collector::TopDocs;
use regex::Regex;

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
        schema_builder.add_text_field("timestamp", TEXT | STORED);
        schema_builder.add_text_field("requestId", TEXT | STORED);
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
            old_man_doc.add_text(self.timestamp, log_data.get("timestamp").unwrap());
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
        let query_parser = QueryParser::for_index(&index, vec![self.timestamp, self.raw]);
        let query = query_parser.parse_query(&query).unwrap();
        let top_docs = searcher.search(&query, &TopDocs::with_limit(1_000)).unwrap();
        top_docs.into_iter().map(move |(_, doc_address)| {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
            let raw = retrieved_doc.get_first(self.raw).unwrap().as_str().unwrap().to_string();
            let timestamp = retrieved_doc.get_first(self.timestamp).unwrap().as_str().unwrap().to_string();
            let request_id = retrieved_doc.get_first(self.request_id).unwrap().as_str().unwrap().to_string();

            let mut record = HashMap::default();
            
            for fv in retrieved_doc {
                record.insert(self.schema.get_field_name(fv.field).to_string(), fv.value.as_ref().as_str().unwrap().to_string());
            }
            record
            }).collect::<Vec<HashMap<String,String>>>()
    }
}
