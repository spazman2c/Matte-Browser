use crate::error::{Error, Result};
use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::*;
use crate::source_map::SourceMapGenerator;

/// JavaScript parser using Pratt parsing technique
pub struct JsParser {
    lexer: Lexer,
    current_token: Option<Token>,
    source_map_generator: SourceMapGenerator,
}

impl JsParser {
    /// Create a new parser for the given source code
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token().ok();
        
        Self {
            lexer,
            current_token,
            source_map_generator: SourceMapGenerator::new(),
        }
    }

    /// Parse the source code into an AST
    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        let position = Position::new(0, 0, 1, 1);
        Ok(Program {
            body: statements,
            position,
        })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token_type() {
            TokenType::Let | TokenType::Const | TokenType::Var => {
                self.parse_variable_declaration()
            }
            TokenType::Function => {
                self.parse_function_declaration()
            }
            TokenType::Class => {
                self.parse_class_declaration()
            }
            TokenType::If => {
                self.parse_if_statement()
            }
            TokenType::For => {
                self.parse_for_statement()
            }
            TokenType::While => {
                self.parse_while_statement()
            }
            TokenType::Return => {
                self.parse_return_statement()
            }
            TokenType::Import => {
                self.parse_import_declaration()
            }
            TokenType::Export => {
                self.parse_export_declaration()
            }
            TokenType::LeftBrace => {
                self.parse_block_statement()
            }
            TokenType::Semicolon => {
                self.parse_empty_statement()
            }
            _ => {
                // Try to parse as expression statement
                let expression = self.parse_expression()?;
                self.expect_semicolon()?;
                
                let position = Position::new(0, 0, 1, 1);
                Ok(Statement::Expression(ExpressionStatement {
                    expression,
                    position,
                }))
            }
        }
    }

    /// Parse a variable declaration
    fn parse_variable_declaration(&mut self) -> Result<Statement> {
        let kind = match self.current_token_type() {
            TokenType::Let => VariableKind::Let,
            TokenType::Const => VariableKind::Const,
            TokenType::Var => VariableKind::Var,
            _ => return Err(Error::syntax(0, "Expected variable declaration keyword")),
        };

        self.advance(); // consume let/const/var

        let mut declarations = Vec::new();
        
        loop {
            let id = self.parse_pattern()?;
            let init = if self.current_token_type() == TokenType::Assign {
                self.advance(); // consume =
                Some(self.parse_expression()?)
            } else {
                None
            };

            let position = Position::new(0, 0, 1, 1);
            declarations.push(VariableDeclarator {
                id,
                init,
                position,
            });

            if self.current_token_type() != TokenType::Comma {
                break;
            }
            self.advance(); // consume comma
        }

        self.expect_semicolon()?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Variable(VariableDeclaration {
            declarations,
            kind,
            position,
        }))
    }

    /// Parse a function declaration
    fn parse_function_declaration(&mut self) -> Result<Statement> {
        self.advance(); // consume 'function'

        let id = if self.current_token_type() == TokenType::Identifier("".to_string()) {
            let name = self.current_token().lexeme.clone();
            self.advance(); // consume identifier
            Some(Identifier {
                name,
                position: Position::new(0, 0, 1, 1),
            })
        } else {
            None
        };

        self.expect(TokenType::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(TokenType::RightParen)?;

        let body = self.parse_block_statement()?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Function(FunctionDeclaration {
            id,
            params,
            body: match body {
                Statement::Block(block) => block,
                _ => return Err(Error::syntax(0, "Expected function body")),
            },
            generator: false,
            async: false,
            position,
        }))
    }

    /// Parse a class declaration
    fn parse_class_declaration(&mut self) -> Result<Statement> {
        self.advance(); // consume 'class'

        let id = if self.current_token_type() == TokenType::Identifier("".to_string()) {
            let name = self.current_token().lexeme.clone();
            self.advance(); // consume identifier
            Some(Identifier {
                name,
                position: Position::new(0, 0, 1, 1),
            })
        } else {
            None
        };

        let super_class = if self.current_token_type() == TokenType::Extends {
            self.advance(); // consume 'extends'
            Some(self.parse_expression()?)
        } else {
            None
        };

        let body = self.parse_class_body()?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Class(ClassDeclaration {
            id,
            super_class,
            body,
            position,
        }))
    }

    /// Parse a class body
    fn parse_class_body(&mut self) -> Result<ClassBody> {
        self.expect(TokenType::LeftBrace)?;

        let mut body = Vec::new();

        while self.current_token_type() != TokenType::RightBrace && !self.is_at_end() {
            let element = self.parse_class_element()?;
            body.push(element);
        }

        self.expect(TokenType::RightBrace)?;

        let position = Position::new(0, 0, 1, 1);
        Ok(ClassBody {
            body,
            position,
        })
    }

    /// Parse a class element
    fn parse_class_element(&mut self) -> Result<ClassElement> {
        // Simplified implementation - just parse as method
        let key = self.parse_expression()?;
        
        if self.current_token_type() == TokenType::LeftParen {
            // Method
            let params = self.parse_parameters()?;
            let function_body = self.parse_block_statement()?;
            
            let value = FunctionExpression {
                id: None,
                params,
                body: match function_body {
                    Statement::Block(block) => block,
                    _ => return Err(Error::syntax(0, "Expected function body")),
                },
                generator: false,
                async: false,
                position: Position::new(0, 0, 1, 1),
            };

            let position = Position::new(0, 0, 1, 1);
            Ok(ClassElement::Method(ClassMethod {
                key,
                value,
                kind: MethodKind::Method,
                computed: false,
                static_: false,
                position,
            }))
        } else {
            // Property
            let value = if self.current_token_type() == TokenType::Assign {
                self.advance(); // consume =
                Some(self.parse_expression()?)
            } else {
                None
            };

            self.expect_semicolon()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(ClassElement::Property(ClassProperty {
                key,
                value,
                computed: false,
                static_: false,
                position,
            }))
        }
    }

    /// Parse an if statement
    fn parse_if_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume 'if'

        self.expect(TokenType::LeftParen)?;
        let test = self.parse_expression()?;
        self.expect(TokenType::RightParen)?;

        let consequent = Box::new(self.parse_statement()?);
        let alternate = if self.current_token_type() == TokenType::Else {
            self.advance(); // consume 'else'
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::If(IfStatement {
            test,
            consequent,
            alternate,
            position,
        }))
    }

    /// Parse a for statement
    fn parse_for_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume 'for'

        self.expect(TokenType::LeftParen)?;

        let init = if self.current_token_type() != TokenType::Semicolon {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        self.expect(TokenType::Semicolon)?;

        let test = if self.current_token_type() != TokenType::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(TokenType::Semicolon)?;

        let update = if self.current_token_type() != TokenType::RightParen {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(TokenType::RightParen)?;

        let body = Box::new(self.parse_statement()?);

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::For(ForStatement {
            init,
            test,
            update,
            body,
            position,
        }))
    }

    /// Parse a while statement
    fn parse_while_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume 'while'

        self.expect(TokenType::LeftParen)?;
        let test = self.parse_expression()?;
        self.expect(TokenType::RightParen)?;

        let body = Box::new(self.parse_statement()?);

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::While(WhileStatement {
            test,
            body,
            position,
        }))
    }

    /// Parse a return statement
    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume 'return'

        let argument = if self.current_token_type() != TokenType::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_semicolon()?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Return(ReturnStatement {
            argument,
            position,
        }))
    }

    /// Parse an import declaration
    fn parse_import_declaration(&mut self) -> Result<Statement> {
        self.advance(); // consume 'import'

        let specifiers = self.parse_import_specifiers()?;

        if self.current_token_type() == TokenType::From {
            self.advance(); // consume 'from'
            let source = self.parse_literal()?;
            self.expect_semicolon()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(Statement::Import(ImportDeclaration {
                specifiers,
                source,
                position,
            }))
        } else {
            Err(Error::syntax(0, "Expected 'from' in import declaration"))
        }
    }

    /// Parse import specifiers
    fn parse_import_specifiers(&mut self) -> Result<Vec<ImportSpecifier>> {
        let mut specifiers = Vec::new();

        if self.current_token_type() == TokenType::Identifier("".to_string()) {
            // Default import
            let local = Identifier {
                name: self.current_token().lexeme.clone(),
                position: Position::new(0, 0, 1, 1),
            };
            self.advance(); // consume identifier

            let position = Position::new(0, 0, 1, 1);
            specifiers.push(ImportSpecifier::Default(ImportDefaultSpecifier {
                local,
                position,
            }));
        } else if self.current_token_type() == TokenType::LeftBrace {
            // Named imports
            self.advance(); // consume '{'

            while self.current_token_type() != TokenType::RightBrace && !self.is_at_end() {
                let local = Identifier {
                    name: self.current_token().lexeme.clone(),
                    position: Position::new(0, 0, 1, 1),
                };
                self.advance(); // consume identifier

                let imported = if self.current_token_type() == TokenType::As {
                    self.advance(); // consume 'as'
                    let imported = Identifier {
                        name: self.current_token().lexeme.clone(),
                        position: Position::new(0, 0, 1, 1),
                    };
                    self.advance(); // consume identifier
                    Some(imported)
                } else {
                    None
                };

                let position = Position::new(0, 0, 1, 1);
                specifiers.push(ImportSpecifier::Named(ImportNamedSpecifier {
                    local,
                    imported,
                    position,
                }));

                if self.current_token_type() == TokenType::Comma {
                    self.advance(); // consume comma
                }
            }

            self.expect(TokenType::RightBrace)?;
        }

        Ok(specifiers)
    }

    /// Parse an export declaration
    fn parse_export_declaration(&mut self) -> Result<Statement> {
        self.advance(); // consume 'export'

        if self.current_token_type() == TokenType::Default {
            self.advance(); // consume 'default'
            let declaration = self.parse_declaration()?;
            self.expect_semicolon()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(Statement::Export(ExportDeclaration::Default(ExportDefaultDeclaration {
                declaration,
                position,
            })))
        } else {
            // Named export
            let declaration = if self.current_token_type() == TokenType::Function 
                || self.current_token_type() == TokenType::Class 
                || self.current_token_type() == TokenType::Let 
                || self.current_token_type() == TokenType::Const 
                || self.current_token_type() == TokenType::Var {
                Some(self.parse_declaration()?)
            } else {
                None
            };

            let specifiers = if declaration.is_none() {
                self.parse_export_specifiers()?
            } else {
                Vec::new()
            };

            let source = if self.current_token_type() == TokenType::From {
                self.advance(); // consume 'from'
                Some(self.parse_literal()?)
            } else {
                None
            };

            if declaration.is_none() && specifiers.is_empty() {
                self.expect_semicolon()?;
            }

            let position = Position::new(0, 0, 1, 1);
            Ok(Statement::Export(ExportDeclaration::Named(ExportNamedDeclaration {
                declaration,
                specifiers,
                source,
                position,
            })))
        }
    }

    /// Parse export specifiers
    fn parse_export_specifiers(&mut self) -> Result<Vec<ExportSpecifier>> {
        let mut specifiers = Vec::new();

        if self.current_token_type() == TokenType::LeftBrace {
            self.advance(); // consume '{'

            while self.current_token_type() != TokenType::RightBrace && !self.is_at_end() {
                let local = Identifier {
                    name: self.current_token().lexeme.clone(),
                    position: Position::new(0, 0, 1, 1),
                };
                self.advance(); // consume identifier

                let exported = if self.current_token_type() == TokenType::As {
                    self.advance(); // consume 'as'
                    let exported = Identifier {
                        name: self.current_token().lexeme.clone(),
                        position: Position::new(0, 0, 1, 1),
                    };
                    self.advance(); // consume identifier
                    exported
                } else {
                    local.clone()
                };

                let position = Position::new(0, 0, 1, 1);
                specifiers.push(ExportSpecifier {
                    exported,
                    local,
                    position,
                });

                if self.current_token_type() == TokenType::Comma {
                    self.advance(); // consume comma
                }
            }

            self.expect(TokenType::RightBrace)?;
        }

        Ok(specifiers)
    }

    /// Parse a block statement
    fn parse_block_statement(&mut self) -> Result<Statement> {
        self.expect(TokenType::LeftBrace)?;

        let mut body = Vec::new();

        while self.current_token_type() != TokenType::RightBrace && !self.is_at_end() {
            let statement = self.parse_statement()?;
            body.push(statement);
        }

        self.expect(TokenType::RightBrace)?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Block(BlockStatement {
            body,
            position,
        }))
    }

    /// Parse an empty statement
    fn parse_empty_statement(&mut self) -> Result<Statement> {
        self.advance(); // consume semicolon

        let position = Position::new(0, 0, 1, 1);
        Ok(Statement::Empty(EmptyStatement {
            position,
        }))
    }

    /// Parse a declaration
    fn parse_declaration(&mut self) -> Result<Declaration> {
        match self.current_token_type() {
            TokenType::Function => {
                let function_decl = self.parse_function_declaration()?;
                match function_decl {
                    Statement::Function(func) => Ok(Declaration::Function(func)),
                    _ => Err(Error::syntax(0, "Expected function declaration")),
                }
            }
            TokenType::Class => {
                let class_decl = self.parse_class_declaration()?;
                match class_decl {
                    Statement::Class(class) => Ok(Declaration::Class(class)),
                    _ => Err(Error::syntax(0, "Expected class declaration")),
                }
            }
            TokenType::Let | TokenType::Const | TokenType::Var => {
                let var_decl = self.parse_variable_declaration()?;
                match var_decl {
                    Statement::Variable(var) => Ok(Declaration::Variable(var)),
                    _ => Err(Error::syntax(0, "Expected variable declaration")),
                }
            }
            _ => Err(Error::syntax(0, "Expected declaration")),
        }
    }

    /// Parse parameters
    fn parse_parameters(&mut self) -> Result<Vec<Pattern>> {
        let mut params = Vec::new();

        if self.current_token_type() != TokenType::RightParen {
            loop {
                let param = self.parse_pattern()?;
                params.push(param);

                if self.current_token_type() != TokenType::Comma {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        Ok(params)
    }

    /// Parse a pattern
    fn parse_pattern(&mut self) -> Result<Pattern> {
        // Simplified - just parse as identifier
        if let TokenType::Identifier(name) = &self.current_token_type() {
            let identifier = Identifier {
                name: name.clone(),
                position: Position::new(0, 0, 1, 1),
            };
            self.advance(); // consume identifier
            Ok(Pattern::Identifier(identifier))
        } else {
            Err(Error::syntax(0, "Expected identifier in pattern"))
        }
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_assignment_expression()
    }

    /// Parse an assignment expression
    fn parse_assignment_expression(&mut self) -> Result<Expression> {
        let left = self.parse_logical_or_expression()?;

        if self.is_assignment_operator() {
            let operator = self.parse_assignment_operator()?;
            self.advance(); // consume assignment operator
            let right = self.parse_assignment_expression()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(Expression::Assignment(AssignmentExpression {
                operator,
                left: self.expression_to_pattern(left)?,
                right,
                position,
            }))
        } else {
            Ok(left)
        }
    }

    /// Parse a logical OR expression
    fn parse_logical_or_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_logical_and_expression()?;

        while self.current_token_type() == TokenType::LogicalOr {
            let operator = LogicalOperator::LogicalOr;
            self.advance(); // consume ||
            let right = self.parse_logical_and_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Logical(LogicalExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse a logical AND expression
    fn parse_logical_and_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_equality_expression()?;

        while self.current_token_type() == TokenType::LogicalAnd {
            let operator = LogicalOperator::LogicalAnd;
            self.advance(); // consume &&
            let right = self.parse_equality_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Logical(LogicalExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse an equality expression
    fn parse_equality_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_relational_expression()?;

        while matches!(self.current_token_type(), 
            TokenType::Equal | TokenType::NotEqual | 
            TokenType::StrictEqual | TokenType::StrictNotEqual) {
            
            let operator = match self.current_token_type() {
                TokenType::Equal => BinaryOperator::Equal,
                TokenType::NotEqual => BinaryOperator::NotEqual,
                TokenType::StrictEqual => BinaryOperator::StrictEqual,
                TokenType::StrictNotEqual => BinaryOperator::StrictNotEqual,
                _ => unreachable!(),
            };
            self.advance(); // consume operator
            let right = self.parse_relational_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Binary(BinaryExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse a relational expression
    fn parse_relational_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_additive_expression()?;

        while matches!(self.current_token_type(), 
            TokenType::LessThan | TokenType::LessThanOrEqual | 
            TokenType::GreaterThan | TokenType::GreaterThanOrEqual) {
            
            let operator = match self.current_token_type() {
                TokenType::LessThan => BinaryOperator::LessThan,
                TokenType::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                TokenType::GreaterThan => BinaryOperator::GreaterThan,
                TokenType::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            self.advance(); // consume operator
            let right = self.parse_additive_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Binary(BinaryExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse an additive expression
    fn parse_additive_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_multiplicative_expression()?;

        while matches!(self.current_token_type(), TokenType::Plus | TokenType::Minus) {
            let operator = match self.current_token_type() {
                TokenType::Plus => BinaryOperator::Plus,
                TokenType::Minus => BinaryOperator::Minus,
                _ => unreachable!(),
            };
            self.advance(); // consume operator
            let right = self.parse_multiplicative_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Binary(BinaryExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse a multiplicative expression
    fn parse_multiplicative_expression(&mut self) -> Result<Expression> {
        let mut left = self.parse_unary_expression()?;

        while matches!(self.current_token_type(), 
            TokenType::Multiply | TokenType::Divide | TokenType::Modulo) {
            
            let operator = match self.current_token_type() {
                TokenType::Multiply => BinaryOperator::Multiply,
                TokenType::Divide => BinaryOperator::Divide,
                TokenType::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance(); // consume operator
            let right = self.parse_unary_expression()?;

            let position = Position::new(0, 0, 1, 1);
            left = Expression::Binary(BinaryExpression {
                operator,
                left,
                right,
                position,
            });
        }

        Ok(left)
    }

    /// Parse a unary expression
    fn parse_unary_expression(&mut self) -> Result<Expression> {
        if self.is_unary_operator() {
            let operator = self.parse_unary_operator()?;
            self.advance(); // consume operator
            let argument = self.parse_unary_expression()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(Expression::Unary(UnaryExpression {
                operator,
                argument,
                prefix: true,
                position,
            }))
        } else {
            self.parse_postfix_expression()
        }
    }

    /// Parse a postfix expression
    fn parse_postfix_expression(&mut self) -> Result<Expression> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            match self.current_token_type() {
                TokenType::LeftParen => {
                    expr = self.parse_call_expression(expr)?;
                }
                TokenType::LeftBracket => {
                    expr = self.parse_member_expression(expr, true)?;
                }
                TokenType::Dot => {
                    expr = self.parse_member_expression(expr, false)?;
                }
                TokenType::Increment | TokenType::Decrement => {
                    let operator = match self.current_token_type() {
                        TokenType::Increment => UpdateOperator::Increment,
                        TokenType::Decrement => UpdateOperator::Decrement,
                        _ => unreachable!(),
                    };
                    self.advance(); // consume operator

                    let position = Position::new(0, 0, 1, 1);
                    expr = Expression::Update(UpdateExpression {
                        operator,
                        argument: expr,
                        prefix: false,
                        position,
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse a primary expression
    fn parse_primary_expression(&mut self) -> Result<Expression> {
        match self.current_token_type() {
            TokenType::Identifier(_) => {
                let name = self.current_token().lexeme.clone();
                self.advance(); // consume identifier
                let position = Position::new(0, 0, 1, 1);
                Ok(Expression::Identifier(Identifier {
                    name,
                    position,
                }))
            }
            TokenType::Number(_) | TokenType::String(_) | TokenType::Boolean(_) | TokenType::Null => {
                let literal = self.parse_literal()?;
                Ok(Expression::Literal(literal))
            }
            TokenType::This => {
                self.advance(); // consume 'this'
                let position = Position::new(0, 0, 1, 1);
                Ok(Expression::This(ThisExpression {
                    position,
                }))
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                self.parse_array_expression()
            }
            TokenType::LeftBrace => {
                self.parse_object_expression()
            }
            _ => Err(Error::syntax(0, "Unexpected token in expression")),
        }
    }

    /// Parse a call expression
    fn parse_call_expression(&mut self, callee: Expression) -> Result<Expression> {
        self.advance(); // consume '('

        let mut arguments = Vec::new();

        if self.current_token_type() != TokenType::RightParen {
            loop {
                let arg = self.parse_expression()?;
                arguments.push(ExpressionOrSpread::Expression(arg));

                if self.current_token_type() != TokenType::Comma {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.expect(TokenType::RightParen)?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Expression::Call(CallExpression {
            callee,
            arguments,
            optional: false,
            position,
        }))
    }

    /// Parse a member expression
    fn parse_member_expression(&mut self, object: Expression, computed: bool) -> Result<Expression> {
        if computed {
            self.advance(); // consume '['
            let property = self.parse_expression()?;
            self.expect(TokenType::RightBracket)?;
        } else {
            self.advance(); // consume '.'
            let name = self.current_token().lexeme.clone();
            self.advance(); // consume identifier
            let position = Position::new(0, 0, 1, 1);
            let property = Expression::Identifier(Identifier {
                name,
                position,
            });
        }

        let position = Position::new(0, 0, 1, 1);
        Ok(Expression::Member(MemberExpression {
            object,
            property: Expression::Identifier(Identifier {
                name: "property".to_string(),
                position: Position::new(0, 0, 1, 1),
            }),
            computed,
            optional: false,
            position,
        }))
    }

    /// Parse an array expression
    fn parse_array_expression(&mut self) -> Result<Expression> {
        self.advance(); // consume '['

        let mut elements = Vec::new();

        if self.current_token_type() != TokenType::RightBracket {
            loop {
                if self.current_token_type() == TokenType::Comma {
                    elements.push(None);
                } else {
                    let element = self.parse_expression()?;
                    elements.push(Some(element));
                }

                if self.current_token_type() != TokenType::Comma {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.expect(TokenType::RightBracket)?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Expression::Array(ArrayExpression {
            elements,
            position,
        }))
    }

    /// Parse an object expression
    fn parse_object_expression(&mut self) -> Result<Expression> {
        self.advance(); // consume '{'

        let mut properties = Vec::new();

        if self.current_token_type() != TokenType::RightBrace {
            loop {
                let property = self.parse_object_property()?;
                properties.push(property);

                if self.current_token_type() != TokenType::Comma {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        self.expect(TokenType::RightBrace)?;

        let position = Position::new(0, 0, 1, 1);
        Ok(Expression::Object(ObjectExpression {
            properties,
            position,
        }))
    }

    /// Parse an object property
    fn parse_object_property(&mut self) -> Result<ObjectProperty> {
        let key = self.parse_expression()?;

        if self.current_token_type() == TokenType::Colon {
            self.advance(); // consume ':'
            let value = self.parse_expression()?;

            let position = Position::new(0, 0, 1, 1);
            Ok(ObjectProperty::Property(Property {
                key,
                value,
                kind: PropertyKind::Init,
                method: false,
                shorthand: false,
                computed: false,
                position,
            }))
        } else {
            // Shorthand property
            let position = Position::new(0, 0, 1, 1);
            Ok(ObjectProperty::Property(Property {
                key: key.clone(),
                value: key,
                kind: PropertyKind::Init,
                method: false,
                shorthand: true,
                computed: false,
                position,
            }))
        }
    }

    /// Parse a literal
    fn parse_literal(&mut self) -> Result<Literal> {
        match self.current_token_type() {
            TokenType::Number(n) => {
                self.advance(); // consume number
                Ok(Literal::Number(*n))
            }
            TokenType::String(s) => {
                self.advance(); // consume string
                Ok(Literal::String(s.clone()))
            }
            TokenType::Boolean(b) => {
                self.advance(); // consume boolean
                Ok(Literal::Boolean(*b))
            }
            TokenType::Null => {
                self.advance(); // consume null
                Ok(Literal::Null)
            }
            _ => Err(Error::syntax(0, "Expected literal")),
        }
    }

    /// Check if current token is an assignment operator
    fn is_assignment_operator(&self) -> bool {
        matches!(self.current_token_type(),
            TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign |
            TokenType::MultiplyAssign | TokenType::DivideAssign | TokenType::ModuloAssign)
    }

    /// Parse an assignment operator
    fn parse_assignment_operator(&mut self) -> Result<AssignmentOperator> {
        match self.current_token_type() {
            TokenType::Assign => Ok(AssignmentOperator::Assign),
            TokenType::PlusAssign => Ok(AssignmentOperator::PlusAssign),
            TokenType::MinusAssign => Ok(AssignmentOperator::MinusAssign),
            TokenType::MultiplyAssign => Ok(AssignmentOperator::MultiplyAssign),
            TokenType::DivideAssign => Ok(AssignmentOperator::DivideAssign),
            TokenType::ModuloAssign => Ok(AssignmentOperator::ModuloAssign),
            _ => Err(Error::syntax(0, "Expected assignment operator")),
        }
    }

    /// Check if current token is a unary operator
    fn is_unary_operator(&self) -> bool {
        matches!(self.current_token_type(),
            TokenType::Plus | TokenType::Minus | TokenType::LogicalNot |
            TokenType::BitwiseNot | TokenType::Increment | TokenType::Decrement)
    }

    /// Parse a unary operator
    fn parse_unary_operator(&mut self) -> Result<UnaryOperator> {
        match self.current_token_type() {
            TokenType::Plus => Ok(UnaryOperator::Plus),
            TokenType::Minus => Ok(UnaryOperator::Minus),
            TokenType::LogicalNot => Ok(UnaryOperator::LogicalNot),
            TokenType::BitwiseNot => Ok(UnaryOperator::BitwiseNot),
            _ => Err(Error::syntax(0, "Expected unary operator")),
        }
    }

    /// Convert expression to pattern
    fn expression_to_pattern(&self, expr: Expression) -> Result<Pattern> {
        match expr {
            Expression::Identifier(id) => Ok(Pattern::Identifier(id)),
            _ => Err(Error::syntax(0, "Expected identifier in assignment target")),
        }
    }

    /// Expect a specific token type
    fn expect(&mut self, expected: TokenType) -> Result<()> {
        if self.current_token_type() == expected {
            self.advance();
            Ok(())
        } else {
            Err(Error::syntax(0, format!("Expected {:?}", expected)))
        }
    }

    /// Expect a semicolon
    fn expect_semicolon(&mut self) -> Result<()> {
        if self.current_token_type() == TokenType::Semicolon {
            self.advance();
        }
        Ok(())
    }

    /// Get current token type
    fn current_token_type(&self) -> &TokenType {
        &self.current_token.as_ref().unwrap().token_type
    }

    /// Get current token
    fn current_token(&self) -> &Token {
        self.current_token.as_ref().unwrap()
    }

    /// Advance to next token
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token().ok();
    }

    /// Check if we're at the end
    fn is_at_end(&self) -> bool {
        matches!(self.current_token_type(), TokenType::Eof)
    }
}
