use std::error::Error;
use std::fmt::Display;
use std::io::Cursor;

use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket::response::{Responder, Response, Result};
use rocket::serde::json::{serde_json, Value};

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub http_status: Status
}

impl ApiError {
    pub fn new(message: &str, http_status: Status) -> Self {
        Self {
          message: String::from(message), 
          http_status 
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiError {}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let err_response = serde_json::to_string(&Value::Object({
          let mut map = serde_json::Map::new();
          map.insert("message".to_string(), Value::String(self.message));
          map.insert("status".to_string(), Value::String(self.http_status.to_string()));

          map
        })).unwrap();

        Response::build()
            .status(self.http_status)
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}