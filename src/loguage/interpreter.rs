use crate::indexer::Indexer;
use log::{error, info, warn};
use std::collections::HashMap;
use std::hash::Hash;
use tree_sitter::Node;

#[derive(Debug)]
pub enum Val {
    Operation,
    LuceneQuery(String),
    SearchResults(HashMap<String, Vec<String>>),
    Nil,
}

pub fn eval(source: &[u8], node: Node, data: &mut HashMap<&str, Val>) -> Val {
    match node.kind() {
        "query" => {
            let cursor = &mut node.walk();
            let operations = node.children(cursor);
            let mut data: HashMap<&str, Val> = HashMap::new();
            for operation in operations {
                if !operation.is_named() {
                    continue;
                }
                let result = eval(&source, operation, &mut data);
                info!("Operation: Value = {}", operation);
                // info!("Operation: Value = {}: {:?}", operation.utf8_text(&source).unwrap(), result);
                data.insert("last_output", result);
            }

            let val = data.remove("last_output");
            println!("outgoing from query. val: {:?}", val);
            val.unwrap_or(Val::Nil)
        }
        "operation" => {
            let operation_name = node.child(0).unwrap().utf8_text(source).unwrap();
            let cursor = &mut node.walk();
            let arguments = node.children(cursor).skip(1);
            if operation_name == "search" {
                // this will be lucene node for sure
                let mut vals = Vec::new();
                for argument in arguments {
                    let argument = argument.child(0).unwrap();
                    let val = match argument.kind() {
                        "lucene_query" => {
                            let expression = argument.utf8_text(source).unwrap();
                            let expression = expression[1..expression.len() - 1].trim(); // remove the ``
                            Val::LuceneQuery(expression.to_string())
                        }
                        "query" => eval(&source, argument, &mut HashMap::new()),
                        _ => {
                            warn!("unsupported search argument: {}", argument);
                            Val::Nil
                        }
                    };

                    vals.push(val);
                }

                let mut compound_lucene_query = String::new();
                for (i, val) in vals.iter().enumerate() {
                    if let Val::LuceneQuery(expression) = val {
                        if (i > 0) {
                            compound_lucene_query += " AND ";
                        }
                        compound_lucene_query += &expression;
                    } else {
                        error!("sub query did not return a lucene query.");
                        return Val::Nil;
                    }
                }

                // TODO: performance implications
                let mut indexer = Indexer::new();
                let result = indexer
                    .query(compound_lucene_query)
                    .into_iter()
                    .collect::<HashMap<String, Vec<String>>>();
                info!("result: {:?}", result);
                Val::SearchResults(result)
            } else if operation_name == "fields" {
                let mut filter_terms = vec![];

                for argument in arguments {
                    // operation_argument -> expression -> term
                    let expression = argument.child(0).unwrap();
                    if expression.kind() != "expression" {
                        warn!(
                            "expected arguments to be expressions. this one isn't : {:?}",
                            expression.utf8_text(&source)
                        );
                        return Val::Nil;
                    }
                    let term = expression.child(0).unwrap();
                    filter_terms.push(term.utf8_text(&source).unwrap());
                }

                let last_output = data.remove("last_output");

                // operate and return
                let result = match last_output {
                    Some(Val::SearchResults(mut last_results)) => {
                        filter_terms.iter().for_each(|term| {
                            if let Some(_) = last_results.remove(*term) {
                                info!("filtered term: {}", term);
                            } else {
                                info!(" did not filter term: {}", term);
                            }
                        });
                        info!("last output: {:?}", last_results);
                        Val::SearchResults(last_results)
                    }
                    _ => {
                        warn!("last output did not have results for fields to work on. it was instead: {:?}", last_output);
                        Val::Nil
                    }
                };
                info!("result: {:?}", result);
                result
            } else if operation_name == "lucene" {
                let last_output = data.remove("last_output");
                let result = match last_output {
                    Some(Val::SearchResults(mut last_results)) => {
                        
                        for (key) in last_results.keys() {
                            let mut term_fields = vec![];
                            for term in list {
                                // TODO: how to escape " in terms
                                let mut term_field = String::new();
                                term_field.push_str(&key);
                                term_field.push_str(":\"");
                                term_field.push_str(&term);
                                term_field.push_str("\"");
                                term_fields.push(term_field);
                            }
                            term_fields.join(" AND ");
                        }

                        Val::Nil
                    }
                    _ => {
                        warn!("last output did not have results for fields to work on. it was instead: {:?}", last_output);
                        Val::Nil
                    }
                };
            } else {
                Val::Nil
            }
        }

        _ => Val::Nil,
    }
}
