use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct CreateUser {
    pub usr: String,
    pub pwd: String,
    pub cpwd: String,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub email: String,
}


#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>
}

impl <T> ApiResponse<T> {
    pub fn successful(data: T) -> Self {
        Self {
            message: "success".to_string(),
            data: Some(data)
        }
    }

    pub fn failed(message: String) -> Self {
        Self {
            message,
            data: None
        }
    }
}