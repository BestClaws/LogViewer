use std::collections::HashMap;
use crate::log_loader;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, SchemaBuilder, Value, STORED, TEXT};
use tantivy::{Document, Index, IndexReader, IndexWriter, ReloadPolicy, Searcher, TantivyDocument};
use tantivy::collector::TopDocs;

pub struct Indexer {
    schema: Schema,
    raw: Field,
    timestamp: Field,
}

impl Indexer {
    pub fn new() -> Self {

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("raw", TEXT | STORED);
        schema_builder.add_text_field("timestamp", TEXT | STORED);
        let schema = schema_builder.build();


        let raw = schema.get_field("raw").unwrap();
        let timestamp = schema.get_field("timestamp").unwrap();

        Self {
            schema,
            raw,
            timestamp
        }
    }

    pub fn index_logfile(&mut self) {

        let index = Index::create_in_dir("./data/indexes", self.schema.clone()).unwrap();
        let mut index_writer: IndexWriter = index.writer(50_000_000).unwrap();

        let lines = log_loader::from_logfile(String::from("./data/test.log"));
        
        for (i, line) in lines.enumerate() {
            let mut old_man_doc = TantivyDocument::default();
            old_man_doc.add_text(self.timestamp, format!("{}", i));
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

            let mut record = HashMap::default();
            record.insert("raw".to_string(), raw);
            record

        }).collect::<Vec<HashMap<String,String>>>()
    }
}
