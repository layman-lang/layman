// core ast definitions - the heart of layman
// working from core ast outward as specified

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Node {
    // statements
    AssignStatement(AssignStatement),
    DeclareStatement(DeclareStatement),
    ConditionalStatement(ConditionalStatement),
    LoopStatement(LoopStatement),
    ReturnStatement(ReturnStatement),
    ImportStatement(ImportStatement),
    ExpressionStatement(ExpressionStatement),
    ThrowStatement(ThrowStatement),
    TryCatchStatement(TryCatchStatement),
    UsingStatement(UsingStatement),
    SetStatement(SetStatement),
    IndexSetStatement(IndexSetStatement),
    StartExpression(StartExpression),
    WaitExpression(WaitExpression),
    RunConcurrentlyStatement(RunConcurrentlyStatement),
    TestStatement(TestStatement),
    InspectStatement(InspectStatement),
    
    // expressions
    LiteralExpression(LiteralExpression),
    VariableExpression(VariableExpression),
    CallExpression(CallExpression),
    OperationExpression(OperationExpression),
    ConditionalExpression(ConditionalExpression),
    AccessExpression(AccessExpression),
    IndexExpression(IndexExpression),
    
    // declarations
    FunctionDeclaration(FunctionDeclaration),
    TypeDeclaration(TypeDeclaration),
    ModuleDeclaration(ModuleDeclaration),
    ClassDeclaration(ClassDeclaration),
    StructDeclaration(StructDeclaration),
    
    // OOP expressions
    ObjectCreation(ObjectCreation),
    MethodCall(MethodCall),
    
    // program root
    Program(Program),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignStatement {
    pub location: Location,
    pub identifier: String,
    pub expression: Box<Node>,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeclareStatement {
    pub location: Location,
    pub name: String,
    pub value: Box<Node>,
    pub type_annotation: Option<Type>,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionalStatement {
    pub location: Location,
    pub condition: Box<Node>,
    pub then_branch: Box<Node>,
    pub else_branch: Option<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoopStatement {
    pub location: Location,
    pub loop_type: LoopType,
    pub condition: Option<Box<Node>>,
    pub iterator: Option<String>,
    pub collection: Option<Box<Node>>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopType {
    ForEach,
    While,
    Repeat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReturnStatement {
    pub location: Location,
    pub expression: Option<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportStatement {
    pub location: Location,
    pub module_name: String,
    pub alias: Option<String>,
    pub specific_imports: Vec<String>,
    pub is_file_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThrowStatement {
    pub location: Location,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TryCatchStatement {
    pub location: Location,
    pub try_block: Box<Node>,
    pub catch_block: Option<Box<Node>>,
    pub error_variable: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetStatement {
    pub location: Location,
    pub property: String,
    pub object: Box<Node>,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexSetStatement {
    pub location: Location,
    pub object: Box<Node>,
    pub index: Box<Node>,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsingStatement {
    pub location: Location,
    pub resource: Box<Node>,
    pub identifier: String,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpressionStatement {
    pub location: Location,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LiteralExpression {
    pub location: Location,
    pub value: LiteralValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),  // changed from Text
    Bool(bool),
    Void,
    Nothing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VariableExpression {
    pub location: Location,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallExpression {
    pub location: Location,
    pub function_name: String,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationExpression {
    pub location: Location,
    pub operator: Operator,
    pub left: Box<Node>,
    pub right: Option<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    // arithmetic
    Plus,
    Minus,
    Times,
    DividedBy,
    Modulo,
    Exists,
    
    // comparison
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    
    // logical
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionalExpression {
    pub location: Location,
    pub condition: Box<Node>,
    pub then_expr: Box<Node>,
    pub else_expr: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccessExpression {
    pub location: Location,
    pub object: Box<Node>,
    pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexExpression {
    pub location: Location,
    pub object: Box<Node>,
    pub index: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionDeclaration {
    pub location: Location,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Box<Node>,
    pub is_async: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDeclaration {
    pub location: Location,
    pub name: String,
    pub exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassDeclaration {
    pub location: Location,
    pub name: String,
    pub extends: Option<String>,
    pub properties: Vec<PropertyDeclaration>,
    pub methods: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructDeclaration {
    pub location: Location,
    pub name: String,
    pub properties: Vec<PropertyDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyDeclaration {
    pub location: Location,
    pub name: String,
    pub type_annotation: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectCreation {
    pub location: Location,
    pub class_name: String,
    pub properties: Vec<PropertyAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyAssignment {
    pub location: Location,
    pub name: String,
    pub value: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MethodCall {
    pub location: Location,
    pub object: Box<Node>,
    pub method_name: String,
    pub arguments: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Program {
    pub location: Location,
    pub statements: Vec<Node>,
}

// types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Type {
    BasicType(BasicType),
    CompositeType(CompositeType),
    FunctionType(Box<FunctionType>),
    GenericType(String),
    ClassType(String), // class name
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BasicType {
    Number,
    String,  // changed from Text
    Bool,    // changed from Boolean
    Void,    // changed from Nothing
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompositeType {
    List(Box<Type>),
    DictionaryType(Box<Type>, Box<Type>), // key type, value type
    Maybe(Box<Type>),
    Tuple(Vec<Type>),
    Set(Box<Type>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
}

impl Node {
    pub fn location(&self) -> Location {
        match self {
            Node::AssignStatement(s) => s.location.clone(),
            Node::DeclareStatement(s) => s.location.clone(),
            Node::ConditionalStatement(s) => s.location.clone(),
            Node::LoopStatement(s) => s.location.clone(),
            Node::ReturnStatement(s) => s.location.clone(),
            Node::ImportStatement(s) => s.location.clone(),
            Node::ExpressionStatement(s) => s.location.clone(),
            Node::ThrowStatement(s) => s.location.clone(),
            Node::TryCatchStatement(s) => s.location.clone(),
            Node::UsingStatement(s) => s.location.clone(),
            Node::SetStatement(s) => s.location.clone(),
            Node::LiteralExpression(e) => e.location.clone(),
            Node::VariableExpression(e) => e.location.clone(),
            Node::CallExpression(e) => e.location.clone(),
            Node::OperationExpression(e) => e.location.clone(),
            Node::ConditionalExpression(e) => e.location.clone(),
            Node::AccessExpression(e) => e.location.clone(),
            Node::IndexExpression(e) => e.location.clone(),
            Node::FunctionDeclaration(d) => d.location.clone(),
            Node::TypeDeclaration(d) => d.location.clone(),
            Node::ModuleDeclaration(d) => d.location.clone(),
            Node::ClassDeclaration(d) => d.location.clone(),
            Node::StructDeclaration(d) => d.location.clone(),
            Node::ObjectCreation(e) => e.location.clone(),
            Node::MethodCall(e) => e.location.clone(),
            Node::Program(p) => p.location.clone(),
            Node::IndexSetStatement(s) => s.location.clone(),
            Node::StartExpression(e) => e.location.clone(),
            Node::WaitExpression(e) => e.location.clone(),
            Node::RunConcurrentlyStatement(s) => s.location.clone(),
            Node::TestStatement(t) => t.location.clone(),
            Node::InspectStatement(s) => s.location.clone(),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Program(p) => write!(f, "Program({} statements)", p.statements.len()),
            Node::AssignStatement(_) => write!(f, "AssignStatement"),
            Node::SetStatement(_) => write!(f, "SetStatement"),
            Node::DeclareStatement(_) => write!(f, "DeclareStatement"),
            _ => write!(f, "{:?}", self),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartExpression {
    pub location: Location,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaitExpression {
    pub location: Location,
    pub expression: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RunConcurrentlyStatement {
    pub location: Location,
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestStatement {
    pub location: Location,
    pub name: String,
    pub body: Box<Node>, // Program
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeDeclaration {
    pub location: Location,
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<(String, String)>, // name, type
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InspectStatement {
    pub location: Location,
    pub expression: Box<Node>,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Case {
    pub variant_name: String,
    pub body: Box<Node>,
}
