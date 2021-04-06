use super::ast;

use super::parser::Parser;
use super::symbol::Symbol;
use super::symbol::SymbolTable;

use std::collections::HashMap;

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
    symbol_table: SymbolTable,
    global_scope: HashMap<String, String>
}

impl<'a> Interpreter<'a> {

    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Interpreter {
            parser: parser,
            symbol_table: SymbolTable::new(),
            global_scope: HashMap::new()
        }
    }

    pub fn interpret(&mut self) {
        let mut ast = self.parser.parse();
        // println!("{}", ast);

        // Build a symbol table
        self.symbol_table.process_abstract_syntax_tree(&mut ast);
        // symbol_table.output();

        // Process root level code block
        self.process_block(&ast.block);

    }

    fn process_block(&mut self, block: &ast::Block) {
        for i in 0..block.get_length() {
            self.process_statement(block.get_statement(i));
        }
    }

    fn process_statement(&mut self, statement: &ast::Statement) {
        match statement {
            ast::Statement::Print(expression) => println!("{}", self.process_expression(&expression)),
            ast::Statement::Let(ident, expression) => self.process_assignment(&ident.symbol, &expression),
            ast::Statement::Assignment(ident, expression) => self.process_assignment(&ident.symbol, &expression),
            ast::Statement::If(if_statement) => match if_statement {
                ast::IfStatement::If(condition, block, other) => self.process_if(condition, block, other),
                ast::IfStatement::ElseIf(condition, block, other) => self.process_if(condition, block, other),
                ast::IfStatement::Else(block) => self.process_block(block)
            },
            ast::Statement::While(condition, block) => {
                while self.process_condition(&condition) {
                    self.process_block(&block);
                }
            }
        }
    }

    fn process_assignment(&mut self, ident: &str, expression: &ast::Expression) {
        let expression = self.process_expression(expression);
        let symbol = self.symbol_table.lookup(ident);

        if symbol.is_none() {
            panic!("Attempted to assign to an unidentified variable - {}", ident);
        }

        self.global_scope.insert(String::from(&symbol.unwrap().name), expression);
    }

    fn process_expression(&mut self, expression: &ast::Expression) -> String {
        match expression {
            ast::Expression::Literal(literal) => self.process_literal(&literal),
            ast::Expression::BinaryOp(bin_op) => self.process_binary_op(&bin_op),
            ast::Expression::UnaryOp(un_op) => {
                match self.process_expression(&un_op.term).parse() {
                    Ok(number) => number,
                    Err(err) => panic!("Invalid number used in binary op - {}", err)
                }
            }
            ast::Expression::Ident(ident) => {
                match self.global_scope.get(&ident.symbol) {
                    Some(val) => String::from(val),
                    None => panic!("Attempted to use a variable before assignment - {}", &ident.symbol)
                }
            },
            _ => panic!("Expression not defined")
        }
    }

    fn process_binary_op(&mut self, binary_op: &ast::BinaryOp) -> String {
        let left_expression: f32 = match self.process_expression(&binary_op.left_term).parse() {
            Ok(number) => number,
            Err(err) => panic!("Invalid number used in binary op - {}", err)
        };

        let right_expression: f32 = match self.process_expression(&binary_op.right_term).parse() {
            Ok(number) => number,
            Err(err) => panic!("Invalid number used in binary op - {}", err)
        };

        match binary_op.operator {
            ast::Operator::Plus => (left_expression + right_expression).to_string(),
            ast::Operator::Minus => (left_expression - right_expression).to_string(),
            ast::Operator::Times => (left_expression * right_expression).to_string(),
            ast::Operator::Divides => (left_expression / right_expression).to_string(),
        }
    }

    fn process_condition(&mut self, condition: &ast::Condition) -> bool {
        let left_expression: f32 = match self.process_expression(&condition.left_expression).parse() {
            Ok(number) => number,
            Err(err) => panic!("Invalid number used in binary op - {}", err)
        };

        let right_expression: f32 = match self.process_expression(&condition.right_expression).parse() {
            Ok(number) => number,
            Err(err) => panic!("Invalid number used in binary op - {}", err)
        };

        match condition.comparator {
            ast::Comparator::Equal => left_expression == right_expression,
            ast::Comparator::NotEqual => left_expression != right_expression,
            ast::Comparator::GreaterThan => left_expression > right_expression,
            ast::Comparator::GreaterThanOrEqual => left_expression >= right_expression,
            ast::Comparator::LessThan => left_expression < right_expression,
            ast::Comparator::LessThanOrEqual => left_expression <= right_expression
        }
    }

    fn process_literal(&mut self, literal: &ast::Literal) -> String {
        match literal {
            ast::Literal::String(s) => String::from(s),
            ast::Literal::Number(s) => String::from(s),
        }
    }

    fn process_if(&mut self, condition: &ast::Condition, block: &ast::Block, other: &Option<Box<ast::IfStatement>>) {
        if self.process_condition(condition) {
            self.process_block(block);
        } else if let Some(else_if_statement) = other {
            self.process_else_if(else_if_statement);
        }
    }

    fn process_else_if(&mut self, else_if: &ast::IfStatement) {
        match else_if {
            ast::IfStatement::If(condition, block, other) => self.process_if(condition, block, other),
            ast::IfStatement::ElseIf(condition, block, other) => self.process_if(condition, block, other),
            ast::IfStatement::Else(block) => self.process_block(block)
        }
    }
}
