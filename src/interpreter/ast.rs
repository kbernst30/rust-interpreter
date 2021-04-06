use std::fmt;

trait NodeOutput {
    fn output(&self, level: usize) -> String;
}

pub struct AbstractSyntaxTree {
    pub block: Block
}

pub struct Block {
    statements: Vec<Statement>
}

pub enum Comparator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEqual,
}

pub enum Operator {
    Plus,
    Minus,
    Times,
    Divides,
}

pub enum Statement {
    Print(Expression),
    Let(Ident, Expression),
    Assignment(Ident, Expression),
    If(IfStatement),
    While(Condition, Block),
}

pub enum IfStatement {
    If(Condition, Block, Option<Box<IfStatement>>),
    ElseIf(Condition, Block, Option<Box<IfStatement>>),
    Else(Block),
}

pub enum Expression {
    Literal(Literal),
    Ident(Ident),
    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>)
}

pub enum Literal {
    String(String),
    Number(String),
}

pub struct Condition {
    pub left_expression: Expression,
    pub comparator: Comparator,
    pub right_expression: Expression,
}

pub struct BinaryOp {
    pub left_term: Expression,
    pub operator: Operator,
    pub right_term: Expression
}

pub struct UnaryOp {
    pub operator: Operator,
    pub term: Expression,
}

pub struct Ident {
    pub symbol: String
}

impl AbstractSyntaxTree {
    pub fn new(block: Block) -> AbstractSyntaxTree {
        AbstractSyntaxTree {
            block: block
        }
    }
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Block {
        Block {
            statements: statements
        }
    }

    pub fn get_statements(&mut self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn get_length(&self) -> usize {
        self.statements.len()
    }

    pub fn get_statement(&self, idx: usize) -> &Statement {
        match self.statements.get(idx) {
            Some(s) => s,
            None => panic!("Invalid index for statement")
        }
    }

    pub fn get_mut_statements(&mut self) -> &mut Vec<Statement> {
        &mut self.statements
    }
}

impl Condition {
    pub fn new(left_expression: Expression, comparator: Comparator, right_expression: Expression) -> Condition {
        Condition {
            left_expression: left_expression,
            comparator: comparator,
            right_expression: right_expression
        }
    }
}

impl BinaryOp {
    pub fn new(left_term: Expression, operator: Operator, right_term: Expression) -> BinaryOp {
        BinaryOp {
            left_term: left_term,
            operator: operator,
            right_term: right_term,
        }
    }
}

impl UnaryOp {
    pub fn new(operator: Operator, term: Expression) -> UnaryOp {
        UnaryOp {
            operator: operator,
            term: term,
        }
    }
}

impl Ident {
    pub fn new(symbol: String) -> Ident {
        Ident { symbol: symbol }
    }
}

impl fmt::Display for AbstractSyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.block.output(0))
    }
}

impl NodeOutput for Block {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&"  ".repeat(level));
        output.push_str("block");
        for statement in &self.statements {
            output.push_str("\n");
            output.push_str(&statement.output(level + 1));
        }
        output
    }
}

impl NodeOutput for Operator {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&"  ".repeat(level));
        output.push_str(match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Divides => "/"
        });
        output
    }
}

impl NodeOutput for Comparator {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&"  ".repeat(level));
        output.push_str(match self {
            Comparator::Equal => "==",
            Comparator::NotEqual => "!=",
            Comparator::GreaterThan => ">",
            Comparator::GreaterThanOrEqual => ">=",
            Comparator::LessThan => "<",
            Comparator::LessThanOrEqual => "<=",
        });
        output
    }
}

impl NodeOutput for Statement {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&"  ".repeat(level));
        match self {
            Statement::Print(expression) => {
                output.push_str("print\n");
                output.push_str(&expression.output(level + 1));
            },
            Statement::Let(ident, expression) => {
                output.push_str("let\n");
                output.push_str(&"  ".repeat(level + 1));
                output.push_str(&ident.symbol);
                output.push_str("\n");
                output.push_str(&expression.output(level + 1));
            },
            Statement::If(if_statement) => output.push_str(&if_statement.output(level)),
            Statement::While(condition, block) => {
                output.push_str("while\n");
                output.push_str(&condition.output(level + 1));
                output.push_str("\n");
                output.push_str(&block.output(level + 1));
            }
            _ => output.push_str("")
        }
        output
    }
}

impl NodeOutput for IfStatement {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        match self {
            IfStatement::If(condition, block, other) => {
                output.push_str("if\n");
                output.push_str(&condition.output(level + 1));
                output.push_str("\n");
                output.push_str(&block.output(level + 1));
                match other {
                    Some(if_statement) => {
                        output.push_str("\n");
                        output.push_str(&if_statement.output(level + 1));
                    },
                    None => {}
                };
            },
            IfStatement::ElseIf(condition, block, other) => {
                output.push_str(&"  ".repeat(level));
                output.push_str("elseif\n");
                output.push_str(&condition.output(level + 1));
                output.push_str("\n");
                output.push_str(&block.output(level + 1));
                match other {
                    Some(if_statement) => {
                        output.push_str("\n");
                        output.push_str(&if_statement.output(level + 1));
                    },
                    None => {}
                };

            },
            IfStatement::Else(block) => {
                output.push_str(&"  ".repeat(level));
                output.push_str("else\n");
                output.push_str(&block.output(level + 1));
            }
        }
        output
    }
}

impl NodeOutput for Expression {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        match self {
            Expression::BinaryOp(op) => output.push_str(&op.output(level)),
            Expression::UnaryOp(op) => output.push_str(&op.term.output(level)),
            Expression::Literal(literal) => output.push_str(&literal.output(level)),
            Expression::Ident(ident) => {
                output.push_str(&"  ".repeat(level));
                output.push_str(&ident.symbol);
            }
        }
        output
    }
}

impl NodeOutput for Condition {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&self.comparator.output(level));
        output.push_str("\n");
        output.push_str(&self.left_expression.output(level + 1));
        output.push_str("\n");
        output.push_str(&self.right_expression.output(level + 1));
        output
    }
}

impl NodeOutput for BinaryOp {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&self.operator.output(level));
        output.push_str("\n");
        output.push_str(&self.left_term.output(level + 1));
        output.push_str("\n");
        output.push_str(&self.right_term.output(level + 1));
        output
    }
}

impl NodeOutput for Literal {
    fn output(&self, level: usize) -> String {
        let mut output = String::new();
        output.push_str(&"  ".repeat(level));
        match self {
            Literal::String(s) => output.push_str(s),
            Literal::Number(s) => output.push_str(s)
        }
        output
    }
}