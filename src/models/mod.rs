use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub info: Info,
    pub models: Vec<Model>,
    pub requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub host: String,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub name: String,
    pub vars: Vec<Box<Variable>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub path: String,
    pub vars: Vec<Box<Variable>>,
    pub method: Method,
    pub response_type: String,
    pub error_type: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get_,
    #[serde(rename = "POST")]
    Post_,
    #[serde(rename = "PUT")]
    Put_,
    #[serde(rename = "DELETE")]
    Delete_,
    #[serde(rename = "OPTIONS")]
    Options_,
    #[serde(rename = "HEAD")]
    Head_,
    #[serde(rename = "PATCH")]
    Patch_,
    #[serde(rename = "TRACE")]
    Trace_,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub variable_type: VariableType,
    pub optional: bool,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum VariableType {
    #[serde(rename = "String")]
    StringType,
    #[serde(rename = "Int")]
    IntType,
    #[serde(rename = "Bool")]
    BoolType,
    #[serde(rename = "Float")]
    FloatType,
    #[serde(rename = "Array")]
    ArrayType(Box<VariableType>),
    #[serde(rename = "Complex")]
    ComplexType(String),
}
