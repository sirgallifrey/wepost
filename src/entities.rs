use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethods {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    OPTION
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDefinition {
    pub path: String,
    pub method: HttpMethods,
    pub headers: HashMap<String, String>,
}

impl RequestDefinition {
    pub fn new(path: String, method: HttpMethods) -> RequestDefinition {
        RequestDefinition {
            path,
            method,
            headers: HashMap::new()
        }
    }
}
