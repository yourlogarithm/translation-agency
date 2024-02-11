use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateUser {
    pub usr: String,
    pub pwd: String,
    pub cpwd: String,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub email: String,
}
