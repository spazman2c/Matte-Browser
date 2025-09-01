use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Position information for AST nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

/// Base trait for all AST nodes
pub trait AstNode {
    fn position(&self) -> &Position;
    fn node_type(&self) -> &str;
}

/// JavaScript program (root node)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Statement>,
    pub position: Position,
}

impl AstNode for Program {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "Program"
    }
}

/// JavaScript statements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Expression(ExpressionStatement),
    Block(BlockStatement),
    If(IfStatement),
    For(ForStatement),
    While(WhileStatement),
    DoWhile(DoWhileStatement),
    Switch(SwitchStatement),
    Try(TryStatement),
    Throw(ThrowStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    Labeled(LabeledStatement),
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Variable(VariableDeclaration),
    Import(ImportDeclaration),
    Export(ExportDeclaration),
    Empty(EmptyStatement),
}

impl AstNode for Statement {
    fn position(&self) -> &Position {
        match self {
            Statement::Expression(stmt) => stmt.position(),
            Statement::Block(stmt) => stmt.position(),
            Statement::If(stmt) => stmt.position(),
            Statement::For(stmt) => stmt.position(),
            Statement::While(stmt) => stmt.position(),
            Statement::DoWhile(stmt) => stmt.position(),
            Statement::Switch(stmt) => stmt.position(),
            Statement::Try(stmt) => stmt.position(),
            Statement::Throw(stmt) => stmt.position(),
            Statement::Return(stmt) => stmt.position(),
            Statement::Break(stmt) => stmt.position(),
            Statement::Continue(stmt) => stmt.position(),
            Statement::Labeled(stmt) => stmt.position(),
            Statement::Function(stmt) => stmt.position(),
            Statement::Class(stmt) => stmt.position(),
            Statement::Variable(stmt) => stmt.position(),
            Statement::Import(stmt) => stmt.position(),
            Statement::Export(stmt) => stmt.position(),
            Statement::Empty(stmt) => stmt.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            Statement::Expression(_) => "ExpressionStatement",
            Statement::Block(_) => "BlockStatement",
            Statement::If(_) => "IfStatement",
            Statement::For(_) => "ForStatement",
            Statement::While(_) => "WhileStatement",
            Statement::DoWhile(_) => "DoWhileStatement",
            Statement::Switch(_) => "SwitchStatement",
            Statement::Try(_) => "TryStatement",
            Statement::Throw(_) => "ThrowStatement",
            Statement::Return(_) => "ReturnStatement",
            Statement::Break(_) => "BreakStatement",
            Statement::Continue(_) => "ContinueStatement",
            Statement::Labeled(_) => "LabeledStatement",
            Statement::Function(_) => "FunctionDeclaration",
            Statement::Class(_) => "ClassDeclaration",
            Statement::Variable(_) => "VariableDeclaration",
            Statement::Import(_) => "ImportDeclaration",
            Statement::Export(_) => "ExportDeclaration",
            Statement::Empty(_) => "EmptyStatement",
        }
    }
}

/// Expression statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub expression: Expression,
    pub position: Position,
}

impl AstNode for ExpressionStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ExpressionStatement"
    }
}

/// Block statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
    pub position: Position,
}

impl AstNode for BlockStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "BlockStatement"
    }
}

/// If statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub test: Expression,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
    pub position: Position,
}

impl AstNode for IfStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "IfStatement"
    }
}

/// For statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForStatement {
    pub init: Option<Box<Statement>>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Box<Statement>,
    pub position: Position,
}

impl AstNode for ForStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ForStatement"
    }
}

/// While statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Box<Statement>,
    pub position: Position,
}

impl AstNode for WhileStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "WhileStatement"
    }
}

/// Do-while statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoWhileStatement {
    pub body: Box<Statement>,
    pub test: Expression,
    pub position: Position,
}

impl AstNode for DoWhileStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "DoWhileStatement"
    }
}

/// Switch statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchStatement {
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
    pub position: Position,
}

impl AstNode for SwitchStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "SwitchStatement"
    }
}

/// Switch case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
    pub position: Position,
}

impl AstNode for SwitchCase {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "SwitchCase"
    }
}

/// Try statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStatement {
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
    pub position: Position,
}

impl AstNode for TryStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "TryStatement"
    }
}

/// Catch clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub param: Option<Pattern>,
    pub body: BlockStatement,
    pub position: Position,
}

impl AstNode for CatchClause {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "CatchClause"
    }
}

/// Throw statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub argument: Expression,
    pub position: Position,
}

impl AstNode for ThrowStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ThrowStatement"
    }
}

/// Return statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub argument: Option<Expression>,
    pub position: Position,
}

impl AstNode for ReturnStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ReturnStatement"
    }
}

/// Break statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
    pub position: Position,
}

impl AstNode for BreakStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "BreakStatement"
    }
}

/// Continue statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
    pub position: Position,
}

impl AstNode for ContinueStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ContinueStatement"
    }
}

/// Labeled statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledStatement {
    pub label: Identifier,
    pub body: Box<Statement>,
    pub position: Position,
}

impl AstNode for LabeledStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "LabeledStatement"
    }
}

/// Function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDeclaration {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: BlockStatement,
    pub generator: bool,
    pub async: bool,
    pub position: Position,
}

impl AstNode for FunctionDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "FunctionDeclaration"
    }
}

/// Class declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDeclaration {
    pub id: Option<Identifier>,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
    pub position: Position,
}

impl AstNode for ClassDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ClassDeclaration"
    }
}

/// Class body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassBody {
    pub body: Vec<ClassElement>,
    pub position: Position,
}

impl AstNode for ClassBody {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ClassBody"
    }
}

/// Class element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassElement {
    Method(ClassMethod),
    Property(ClassProperty),
    PrivateMethod(PrivateMethod),
    PrivateProperty(PrivateProperty),
}

impl AstNode for ClassElement {
    fn position(&self) -> &Position {
        match self {
            ClassElement::Method(method) => method.position(),
            ClassElement::Property(prop) => prop.position(),
            ClassElement::PrivateMethod(method) => method.position(),
            ClassElement::PrivateProperty(prop) => prop.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            ClassElement::Method(_) => "ClassMethod",
            ClassElement::Property(_) => "ClassProperty",
            ClassElement::PrivateMethod(_) => "PrivateMethod",
            ClassElement::PrivateProperty(_) => "PrivateProperty",
        }
    }
}

/// Class method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMethod {
    pub key: Expression,
    pub value: FunctionExpression,
    pub kind: MethodKind,
    pub computed: bool,
    pub static_: bool,
    pub position: Position,
}

impl AstNode for ClassMethod {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ClassMethod"
    }
}

/// Method kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethodKind {
    Constructor,
    Method,
    Get,
    Set,
}

/// Class property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassProperty {
    pub key: Expression,
    pub value: Option<Expression>,
    pub computed: bool,
    pub static_: bool,
    pub position: Position,
}

impl AstNode for ClassProperty {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ClassProperty"
    }
}

/// Private method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMethod {
    pub key: PrivateName,
    pub value: FunctionExpression,
    pub kind: MethodKind,
    pub static_: bool,
    pub position: Position,
}

impl AstNode for PrivateMethod {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "PrivateMethod"
    }
}

/// Private property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateProperty {
    pub key: PrivateName,
    pub value: Option<Expression>,
    pub static_: bool,
    pub position: Position,
}

impl AstNode for PrivateProperty {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "PrivateProperty"
    }
}

/// Private name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateName {
    pub id: Identifier,
    pub position: Position,
}

impl AstNode for PrivateName {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "PrivateName"
    }
}

/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
    pub position: Position,
}

impl AstNode for VariableDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "VariableDeclaration"
    }
}

/// Variable kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

/// Variable declarator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<Expression>,
    pub position: Position,
}

impl AstNode for VariableDeclarator {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "VariableDeclarator"
    }
}

/// Import declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDeclaration {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
    pub position: Position,
}

impl AstNode for ImportDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ImportDeclaration"
    }
}

/// Import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSpecifier {
    Default(ImportDefaultSpecifier),
    Named(ImportNamedSpecifier),
    Namespace(ImportNamespaceSpecifier),
}

impl AstNode for ImportSpecifier {
    fn position(&self) -> &Position {
        match self {
            ImportSpecifier::Default(spec) => spec.position(),
            ImportSpecifier::Named(spec) => spec.position(),
            ImportSpecifier::Namespace(spec) => spec.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            ImportSpecifier::Default(_) => "ImportDefaultSpecifier",
            ImportSpecifier::Named(_) => "ImportNamedSpecifier",
            ImportSpecifier::Namespace(_) => "ImportNamespaceSpecifier",
        }
    }
}

/// Import default specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDefaultSpecifier {
    pub local: Identifier,
    pub position: Position,
}

impl AstNode for ImportDefaultSpecifier {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ImportDefaultSpecifier"
    }
}

/// Import named specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportNamedSpecifier {
    pub local: Identifier,
    pub imported: Option<Identifier>,
    pub position: Position,
}

impl AstNode for ImportNamedSpecifier {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ImportNamedSpecifier"
    }
}

/// Import namespace specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportNamespaceSpecifier {
    pub local: Identifier,
    pub position: Position,
}

impl AstNode for ImportNamespaceSpecifier {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ImportNamespaceSpecifier"
    }
}

/// Export declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDeclaration {
    Named(ExportNamedDeclaration),
    Default(ExportDefaultDeclaration),
    All(ExportAllDeclaration),
}

impl AstNode for ExportDeclaration {
    fn position(&self) -> &Position {
        match self {
            ExportDeclaration::Named(decl) => decl.position(),
            ExportDeclaration::Default(decl) => decl.position(),
            ExportDeclaration::All(decl) => decl.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            ExportDeclaration::Named(_) => "ExportNamedDeclaration",
            ExportDeclaration::Default(_) => "ExportDefaultDeclaration",
            ExportDeclaration::All(_) => "ExportAllDeclaration",
        }
    }
}

/// Export named declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportNamedDeclaration {
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
    pub position: Position,
}

impl AstNode for ExportNamedDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ExportNamedDeclaration"
    }
}

/// Export default declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDefaultDeclaration {
    pub declaration: Declaration,
    pub position: Position,
}

impl AstNode for ExportDefaultDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ExportDefaultDeclaration"
    }
}

/// Export all declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportAllDeclaration {
    pub source: Literal,
    pub position: Position,
}

impl AstNode for ExportAllDeclaration {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ExportAllDeclaration"
    }
}

/// Export specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSpecifier {
    pub exported: Identifier,
    pub local: Identifier,
    pub position: Position,
}

impl AstNode for ExportSpecifier {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ExportSpecifier"
    }
}

/// Empty statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyStatement {
    pub position: Position,
}

impl AstNode for EmptyStatement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "EmptyStatement"
    }
}

/// JavaScript expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    This(ThisExpression),
    Array(ArrayExpression),
    Object(ObjectExpression),
    Function(FunctionExpression),
    ArrowFunction(ArrowFunctionExpression),
    Class(ClassExpression),
    TaggedTemplate(TaggedTemplateExpression),
    Member(MemberExpression),
    Super(Super),
    MetaProperty(MetaProperty),
    Call(CallExpression),
    New(NewExpression),
    Update(UpdateExpression),
    Await(AwaitExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Logical(LogicalExpression),
    Conditional(ConditionalExpression),
    Yield(YieldExpression),
    Assignment(AssignmentExpression),
    Sequence(SequenceExpression),
}

impl AstNode for Expression {
    fn position(&self) -> &Position {
        match self {
            Expression::Identifier(expr) => expr.position(),
            Expression::Literal(expr) => expr.position(),
            Expression::This(expr) => expr.position(),
            Expression::Array(expr) => expr.position(),
            Expression::Object(expr) => expr.position(),
            Expression::Function(expr) => expr.position(),
            Expression::ArrowFunction(expr) => expr.position(),
            Expression::Class(expr) => expr.position(),
            Expression::TaggedTemplate(expr) => expr.position(),
            Expression::Member(expr) => expr.position(),
            Expression::Super(expr) => expr.position(),
            Expression::MetaProperty(expr) => expr.position(),
            Expression::Call(expr) => expr.position(),
            Expression::New(expr) => expr.position(),
            Expression::Update(expr) => expr.position(),
            Expression::Await(expr) => expr.position(),
            Expression::Unary(expr) => expr.position(),
            Expression::Binary(expr) => expr.position(),
            Expression::Logical(expr) => expr.position(),
            Expression::Conditional(expr) => expr.position(),
            Expression::Yield(expr) => expr.position(),
            Expression::Assignment(expr) => expr.position(),
            Expression::Sequence(expr) => expr.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            Expression::Identifier(_) => "Identifier",
            Expression::Literal(_) => "Literal",
            Expression::This(_) => "ThisExpression",
            Expression::Array(_) => "ArrayExpression",
            Expression::Object(_) => "ObjectExpression",
            Expression::Function(_) => "FunctionExpression",
            Expression::ArrowFunction(_) => "ArrowFunctionExpression",
            Expression::Class(_) => "ClassExpression",
            Expression::TaggedTemplate(_) => "TaggedTemplateExpression",
            Expression::Member(_) => "MemberExpression",
            Expression::Super(_) => "Super",
            Expression::MetaProperty(_) => "MetaProperty",
            Expression::Call(_) => "CallExpression",
            Expression::New(_) => "NewExpression",
            Expression::Update(_) => "UpdateExpression",
            Expression::Await(_) => "AwaitExpression",
            Expression::Unary(_) => "UnaryExpression",
            Expression::Binary(_) => "BinaryExpression",
            Expression::Logical(_) => "LogicalExpression",
            Expression::Conditional(_) => "ConditionalExpression",
            Expression::Yield(_) => "YieldExpression",
            Expression::Assignment(_) => "AssignmentExpression",
            Expression::Sequence(_) => "SequenceExpression",
        }
    }
}

/// Identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

impl AstNode for Identifier {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "Identifier"
    }
}

/// Literal values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    RegExp(RegExpLiteral),
    Template(TemplateLiteral),
}

impl AstNode for Literal {
    fn position(&self) -> &Position {
        // This is a simplified implementation
        // In a real implementation, each literal would store its position
        &Position::new(0, 0, 0, 0)
    }

    fn node_type(&self) -> &str {
        "Literal"
    }
}

/// Regular expression literal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegExpLiteral {
    pub pattern: String,
    pub flags: String,
    pub position: Position,
}

impl AstNode for RegExpLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "RegExpLiteral"
    }
}

/// Template literal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLiteral {
    pub quasis: Vec<TemplateElement>,
    pub expressions: Vec<Expression>,
    pub position: Position,
}

impl AstNode for TemplateLiteral {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "TemplateLiteral"
    }
}

/// Template element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement {
    pub value: TemplateElementValue,
    pub tail: bool,
    pub position: Position,
}

impl AstNode for TemplateElement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "TemplateElement"
    }
}

/// Template element value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElementValue {
    pub raw: String,
    pub cooked: String,
}

/// This expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThisExpression {
    pub position: Position,
}

impl AstNode for ThisExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ThisExpression"
    }
}

/// Array expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Option<Expression>>,
    pub position: Position,
}

impl AstNode for ArrayExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ArrayExpression"
    }
}

/// Object expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectExpression {
    pub properties: Vec<ObjectProperty>,
    pub position: Position,
}

impl AstNode for ObjectExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ObjectExpression"
    }
}

/// Object property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectProperty {
    Property(Property),
    SpreadElement(SpreadElement),
}

impl AstNode for ObjectProperty {
    fn position(&self) -> &Position {
        match self {
            ObjectProperty::Property(prop) => prop.position(),
            ObjectProperty::SpreadElement(spread) => spread.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            ObjectProperty::Property(_) => "Property",
            ObjectProperty::SpreadElement(_) => "SpreadElement",
        }
    }
}

/// Property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub key: Expression,
    pub value: Expression,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
    pub position: Position,
}

impl AstNode for Property {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "Property"
    }
}

/// Property kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

/// Spread element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadElement {
    pub argument: Expression,
    pub position: Position,
}

impl AstNode for SpreadElement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "SpreadElement"
    }
}

/// Function expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: BlockStatement,
    pub generator: bool,
    pub async: bool,
    pub position: Position,
}

impl AstNode for FunctionExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "FunctionExpression"
    }
}

/// Arrow function expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Pattern>,
    pub body: ArrowFunctionBody,
    pub async: bool,
    pub position: Position,
}

impl AstNode for ArrowFunctionExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ArrowFunctionExpression"
    }
}

/// Arrow function body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowFunctionBody {
    Block(BlockStatement),
    Expression(Box<Expression>),
}

/// Class expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassExpression {
    pub id: Option<Identifier>,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
    pub position: Position,
}

impl AstNode for ClassExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ClassExpression"
    }
}

/// Tagged template expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggedTemplateExpression {
    pub tag: Expression,
    pub quasi: TemplateLiteral,
    pub position: Position,
}

impl AstNode for TaggedTemplateExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "TaggedTemplateExpression"
    }
}

/// Member expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpression {
    pub object: Expression,
    pub property: Expression,
    pub computed: bool,
    pub optional: bool,
    pub position: Position,
}

impl AstNode for MemberExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "MemberExpression"
    }
}

/// Super
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Super {
    pub position: Position,
}

impl AstNode for Super {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "Super"
    }
}

/// Meta property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaProperty {
    pub meta: Identifier,
    pub property: Identifier,
    pub position: Position,
}

impl AstNode for MetaProperty {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "MetaProperty"
    }
}

/// Call expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Expression,
    pub arguments: Vec<ExpressionOrSpread>,
    pub optional: bool,
    pub position: Position,
}

impl AstNode for CallExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "CallExpression"
    }
}

/// Expression or spread
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionOrSpread {
    Expression(Expression),
    Spread(SpreadElement),
}

/// New expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub callee: Expression,
    pub arguments: Vec<ExpressionOrSpread>,
    pub position: Position,
}

impl AstNode for NewExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "NewExpression"
    }
}

/// Update expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExpression {
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
    pub position: Position,
}

impl AstNode for UpdateExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "UpdateExpression"
    }
}

/// Update operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

/// Await expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwaitExpression {
    pub argument: Expression,
    pub position: Position,
}

impl AstNode for AwaitExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "AwaitExpression"
    }
}

/// Unary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub argument: Expression,
    pub prefix: bool,
    pub position: Position,
}

impl AstNode for UnaryExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "UnaryExpression"
    }
}

/// Unary operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
    TypeOf,
    Void,
    Delete,
}

/// Binary expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
    pub position: Position,
}

impl AstNode for BinaryExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "BinaryExpression"
    }
}

/// Binary operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LogicalAnd,
    LogicalOr,
    In,
    InstanceOf,
}

/// Logical expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalExpression {
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
    pub position: Position,
}

impl AstNode for LogicalExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "LogicalExpression"
    }
}

/// Logical operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
}

/// Conditional expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub test: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
    pub position: Position,
}

impl AstNode for ConditionalExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ConditionalExpression"
    }
}

/// Yield expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldExpression {
    pub argument: Option<Expression>,
    pub delegate: bool,
    pub position: Position,
}

impl AstNode for YieldExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "YieldExpression"
    }
}

/// Assignment expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub operator: AssignmentOperator,
    pub left: Pattern,
    pub right: Expression,
    pub position: Position,
}

impl AstNode for AssignmentExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "AssignmentExpression"
    }
}

/// Assignment operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentOperator {
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    ExponentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    NullishCoalescingAssign,
}

/// Sequence expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceExpression {
    pub expressions: Vec<Expression>,
    pub position: Position,
}

impl AstNode for SequenceExpression {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "SequenceExpression"
    }
}

/// JavaScript patterns (for destructuring)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Rest(RestElement),
    Assignment(AssignmentPattern),
}

impl AstNode for Pattern {
    fn position(&self) -> &Position {
        match self {
            Pattern::Identifier(pat) => pat.position(),
            Pattern::Object(pat) => pat.position(),
            Pattern::Array(pat) => pat.position(),
            Pattern::Rest(pat) => pat.position(),
            Pattern::Assignment(pat) => pat.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            Pattern::Identifier(_) => "Identifier",
            Pattern::Object(_) => "ObjectPattern",
            Pattern::Array(_) => "ArrayPattern",
            Pattern::Rest(_) => "RestElement",
            Pattern::Assignment(_) => "AssignmentPattern",
        }
    }
}

/// Object pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPattern {
    pub properties: Vec<ObjectPatternProperty>,
    pub position: Position,
}

impl AstNode for ObjectPattern {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ObjectPattern"
    }
}

/// Object pattern property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectPatternProperty {
    Property(ObjectPatternProperty),
    RestElement(RestElement),
}

impl AstNode for ObjectPatternProperty {
    fn position(&self) -> &Position {
        match self {
            ObjectPatternProperty::Property(prop) => prop.position(),
            ObjectPatternProperty::RestElement(rest) => rest.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            ObjectPatternProperty::Property(_) => "ObjectPatternProperty",
            ObjectPatternProperty::RestElement(_) => "RestElement",
        }
    }
}

/// Object pattern property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPatternProperty {
    pub key: Expression,
    pub value: Pattern,
    pub computed: bool,
    pub shorthand: bool,
    pub position: Position,
}

impl AstNode for ObjectPatternProperty {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ObjectPatternProperty"
    }
}

/// Array pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayPattern {
    pub elements: Vec<Option<Pattern>>,
    pub position: Position,
}

impl AstNode for ArrayPattern {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "ArrayPattern"
    }
}

/// Rest element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestElement {
    pub argument: Pattern,
    pub position: Position,
}

impl AstNode for RestElement {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "RestElement"
    }
}

/// Assignment pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentPattern {
    pub left: Pattern,
    pub right: Expression,
    pub position: Position,
}

impl AstNode for AssignmentPattern {
    fn position(&self) -> &Position {
        &self.position
    }

    fn node_type(&self) -> &str {
        "AssignmentPattern"
    }
}

/// JavaScript declarations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    Function(FunctionDeclaration),
    Class(ClassDeclaration),
    Variable(VariableDeclaration),
}

impl AstNode for Declaration {
    fn position(&self) -> &Position {
        match self {
            Declaration::Function(decl) => decl.position(),
            Declaration::Class(decl) => decl.position(),
            Declaration::Variable(decl) => decl.position(),
        }
    }

    fn node_type(&self) -> &str {
        match self {
            Declaration::Function(_) => "FunctionDeclaration",
            Declaration::Class(_) => "ClassDeclaration",
            Declaration::Variable(_) => "VariableDeclaration",
        }
    }
}
