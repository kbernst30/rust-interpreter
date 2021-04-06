use super::ast;

use super::lexer::Lexer;

use super::token::Token;
use super::token::TokenType;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lexer,
            current_token: Token::new(TokenType::ILLEGAL, String::from("")),
            next_token: Token::new(TokenType::ILLEGAL, String::from("")),
        }
    }

    pub fn parse(&mut self) -> ast::AbstractSyntaxTree {
        // Process twice on first parse - this will ensure next and current are both set
        self.process_next();
        self.process_next();

        ast::AbstractSyntaxTree::new(self.parse_program())
    }

    fn parse_program(&mut self) -> ast::Block {
        let mut statements: Vec<ast::Statement> = Vec::new();

        while !self.check_token(&TokenType::EOF) {
            statements.push(self.parse_statement());
        }

        ast::Block::new(statements)
    }

    fn parse_statement(&mut self) -> ast::Statement {
        match self.current_token.get_token_type() {
            TokenType::PRINT => {
                self.process_next();
                let statement = ast::Statement::Print(self.parse_expression());
                self.match_token(TokenType::SEMICOLON);
                statement
            },
            TokenType::LET => {
                self.process_next();
                // The next token should be an IDENT token - add it to variables
                // If IDENT isn't next, the parser will error out anyways
                let ident = ast::Ident::new(String::from(self.current_token.get_token_text()));
                self.match_token(TokenType::IDENT);
                self.match_token(TokenType::EQ);
                let statement = ast::Statement::Let(ident, self.parse_expression());
                self.match_token(TokenType::SEMICOLON);
                statement
            },
            TokenType::IDENT => {
                let ident = ast::Ident::new(String::from(self.current_token.get_token_text()));
                self.match_token(TokenType::IDENT);
                self.match_token(TokenType::EQ);
                let statement = ast::Statement::Assignment(ident, self.parse_expression());
                self.match_token(TokenType::SEMICOLON);
                statement
            }
            TokenType::IF => {
                ast::Statement::If(self.parse_if())
            }
            TokenType::WHILE => {
                self.process_next();

                let condition = self.parse_condition();
                self.match_token(TokenType::THEN);

                let mut statements: Vec<ast::Statement> = Vec::new();
                while !self.check_token(&TokenType::END) {
                    statements.push(self.parse_statement());
                }

                self.match_token(TokenType::END);
                let block = ast::Block::new(statements);

                ast::Statement::While(condition, block)

            },
            _ => panic!("Invalid statement found - {:?}", self.current_token.get_token_type())
        }
    }

    fn parse_condition(&mut self) -> ast::Condition {
        let left_expression = self.parse_expression();
        let comparator = match self.current_token.get_token_type() {
            TokenType::EQEQ => ast::Comparator::Equal,
            TokenType::NOTEQ => ast::Comparator::NotEqual,
            TokenType::GT => ast::Comparator::GreaterThan,
            TokenType::GTEQ => ast::Comparator::GreaterThanOrEqual,
            TokenType::LT => ast::Comparator::LessThan,
            TokenType::LTEQ => ast::Comparator::LessThanOrEqual,
            _ => panic!("Expected comparison operator to evaluate to bool")
        };

        self.process_next();
        let right_expression = self.parse_expression();

        // TODO multiple sequential conditions

        ast::Condition::new(left_expression, comparator, right_expression)
    }

    fn parse_expression(&mut self) -> ast::Expression {
        match self.current_token.get_token_type() {
            TokenType::STRING => {
                let literal = ast::Literal::String(String::from(self.current_token.get_token_text()));
                let expression = ast::Expression::Literal(literal);
                self.process_next();
                expression
            },
            _ => {
                let mut term = self.parse_term();
                while self.check_token(&TokenType::PLUS) || self.check_token(&TokenType::MINUS) {
                    let operator = match self.current_token.get_token_type() {
                        TokenType::PLUS => ast::Operator::Plus,
                        TokenType::MINUS => ast::Operator::Minus,
                        _ => panic!("Invalid operator found")
                    };

                    self.process_next();
                    let first_term = term;
                    let second_term = self.parse_term();
                    let binary_op = ast::BinaryOp::new(first_term, operator, second_term);
                    term = ast::Expression::BinaryOp(Box::new(binary_op));
                }

                term
            }
        }
    }

    fn parse_term(&mut self) -> ast::Expression {
        let mut unary = self.parse_unary();
        while self.check_token(&TokenType::SLASH) || self.check_token(&TokenType::ASTERISK) {
            let operator = match self.current_token.get_token_type() {
                TokenType::ASTERISK => ast::Operator::Times,
                TokenType::SLASH => ast::Operator::Divides,
                _ => panic!("Invalid operator found")
            };

            self.process_next();
            let first_unary = unary;
            let second_unary = self.parse_unary();
            let binary_op = ast::BinaryOp::new(first_unary, operator, second_unary);
            unary = ast::Expression::BinaryOp(Box::new(binary_op));
        }

        unary
    }

    fn parse_unary(&mut self) -> ast::Expression {
        match self.current_token.get_token_type() {
            TokenType::PLUS => {
                // Unary can start with + or - but it is not required
                self.process_next();
                let unary_op = ast::UnaryOp::new(ast::Operator::Plus, self.parse_primary());
                ast::Expression::UnaryOp(Box::new(unary_op))
            },
            TokenType::MINUS => {
                // Unary can start with + or - but it is not required
                self.process_next();
                let unary_op = ast::UnaryOp::new(ast::Operator::Minus, self.parse_primary());
                ast::Expression::UnaryOp(Box::new(unary_op))
            },
            _ => self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> ast::Expression {
        let primary = match self.current_token.get_token_type() {
            TokenType::NUMBER => {
                let number = String::from(self.current_token.get_token_text());
                ast::Expression::Literal(ast::Literal::Number(number))
            },
            TokenType::IDENT => {
                let ident = ast::Ident::new(String::from(self.current_token.get_token_text()));
                ast::Expression::Ident(ident)
            },
            _ => panic!("Syntax Error! Expected number of ident")
        };

        self.process_next();
        primary
    }

    fn parse_if(&mut self) -> ast::IfStatement {
        let current_token_type = self.current_token.get_token_type().clone();

        self.process_next();
        let mut condition: Option<ast::Condition> = None;
        if current_token_type != TokenType::ELSE {
            condition = Some(self.parse_condition());
            self.match_token(TokenType::THEN);
        }

        let mut statements: Vec<ast::Statement> = Vec::new();

        // We can have lots of statements inside our IF block - so loop until we find an END
        let mut other: Option<Box<ast::IfStatement>> = None;
        while !self.check_token(&TokenType::END) {
            // If it's an ELSEIF or ELSE statement, we need to recurseively parse our IF
            if self.check_token(&TokenType::ELSEIF) || self.check_token(&TokenType::ELSE) {
                // ELSE must be last so if we already ARE an ELSE and we find another, panic
                if current_token_type == TokenType::ELSE {
                    panic!("Invalid elseif");
                }

                other = Some(Box::new(self.parse_if()));

            } else {
                statements.push(self.parse_statement());
            }
        }

        // Only force Match END once - if this was the first IF statement
        if current_token_type == TokenType::IF {
            self.match_token(TokenType::END);
        }

        let block = ast::Block::new(statements);
        match current_token_type {
            TokenType::IF => ast::IfStatement::If(condition.unwrap(), block, other),
            TokenType::ELSEIF => ast::IfStatement::ElseIf(condition.unwrap(), block, other),
            TokenType::ELSE => ast::IfStatement::Else(block),
            _ => panic!("Invalid IF statement constructed")
        }
    }

    fn process_next(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.get_token();
    }

    fn match_token(&mut self, token_type: TokenType) {
        if !self.check_token(&token_type) {
            panic!("Syntax error! - Expected {:?} found {:?}", token_type, self.current_token.get_token_type());
        }

        // Match was successful, advance to next token
        self.process_next();
    }

    fn check_token(&mut self, token_type: &TokenType) -> bool {
        self.current_token.get_token_type() == token_type
    }

}