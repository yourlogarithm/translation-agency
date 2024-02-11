use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: Option<T>
}

impl <T> ApiResponse<T> {
    pub fn successful(data: T) -> Self {
        Self {
            message: "success".to_owned(),
            data: Some(data)
        }
    }

    pub fn failed_str(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            data: None
        }
    }

    pub fn failed_string(message: String) -> Self {
        Self {
            message,
            data: None
        }
    }
}
