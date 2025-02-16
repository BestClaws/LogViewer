use std::collections::HashMap;
use log::warn;
use tree_sitter::Node;

#[derive(Debug)]
pub enum Val {
    Operation,
    LuceneQuery(String),
    Nil,
}

pub fn eval(source: &[u8], node: Node, data: &HashMap<&str, Val>) -> Val {
    match node.kind() {
        "query" => {
            let cursor = &mut node.walk();
            let operations = node.children(cursor);
            let mut data: HashMap<&str, Val> = HashMap::new();
            for operation in operations {
                if !operation.is_named() {
                    continue;
                }
                let result = eval(&source, operation, &data);
                data.insert("last_output", result);
            }

            let val = data.remove("last_output");
            println!("outgoing from query. val: {:?}", val);
            val.unwrap_or(Val::Nil)
        }
        "operation" => {
            let operation_name = node.child(0).unwrap().utf8_text(source).unwrap();

            if operation_name == "search" {
                // this will be lucene node for sure
                let cursor = &mut node.walk();
                let mut vals = Vec::new();
                for argument in node.children(cursor).skip(1) {
                    let argument = argument.child(0).unwrap();
                    let val = match argument.kind() {
                        "lucene_query" => {
                            let expression = argument.utf8_text(source).unwrap();
                            Val::LuceneQuery(expression.to_string())
                        }
                        "query" => {
                            let result = eval(&source, argument, &HashMap::new());
                            if let Val::LuceneQuery(s) = result {
                                Val::LuceneQuery(s)
                            } else {
                                warn!("sub query did not return a lucene query.");
                                Val::Nil
                            }

                        },
                        _ => Val::Nil,
                    };

                    vals.push(val);
                }

                let mut compound = String::new();
                for val in vals {
                    if let Val::LuceneQuery(expression) = val {
                        compound += " AND ";
                        compound += &expression;
                    }
                }

                Val::LuceneQuery(compound)
            } else {
                Val::Nil
            }

        }

        _ => Val::Nil,
    }
}