use super::ast;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: String
}

impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name: name }
    }
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: HashMap::new() }
    }

    pub fn process_abstract_syntax_tree(&mut self, ast: &mut ast::AbstractSyntaxTree) {
        self.process_block(&ast.block);
    }

    pub fn define_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn lookup(&mut self, key: &str) -> Option<&Symbol> {
        self.symbols.get(key)
    }

    pub fn output(&mut self) {
        for key in self.symbols.keys() {
            println!("{}", key);
        }
    }

    fn process_block(&mut self, block: &ast::Block) {
        for i in 0..block.get_length() {
            self.process_statement(block.get_statement(i));
        }
    }

    fn process_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Let(ident, _) => {
                let symbol = Symbol { name: ident.symbol.clone() };
                self.define_symbol(symbol);
            },
            ast::Statement::Assignment(ident, _) => {
                let symbol = self.lookup(&ident.symbol);
                if symbol.is_none() {
                    panic!("Referenced symbol {} before assignment", &ident.symbol);
                }
            },
            ast::Statement::If(if_statement) => {
                match if_statement {
                    ast::IfStatement::If(_, block, _) => self.process_block(block),
                    ast::IfStatement::ElseIf(_, block, _) => self.process_block(block),
                    ast::IfStatement::Else(block) => self.process_block(block)
                }
            },
            ast::Statement::While(_, block) => self.process_block(block),
            _ => {}
        }
    }
}