use crate::loguage::interpreter::{eval, Val};
use std::collections::HashMap;
use tree_sitter::Parser;

pub struct Loguage {
    parser: Parser,
}

impl Loguage {
    pub fn new() -> Loguage {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_loguage::LANGUAGE.into())
            .expect("Error loading Rust grammar");
        Loguage { parser }
    }

    pub fn exec(&mut self, source_code: &str) -> Val {
        let tree = self.parser.parse(source_code, None).unwrap();
        eval(source_code.as_bytes(), tree.root_node(), &HashMap::new())
    }
}
