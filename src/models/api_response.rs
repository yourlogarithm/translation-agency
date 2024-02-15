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

    pub fn failed<S>(message: S) -> Self
    where
        S: ToString
    {
        Self {
            message: message.to_string(),
            data: None
        }
    }
}
