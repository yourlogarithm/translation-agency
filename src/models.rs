use serde::Deserialize;


#[derive(Deserialize)]
pub struct CreateUser {
    pub usr: String,
    pub pwd: String,
    pub cpwd: String,
    pub fname: String,
    pub lname: String,
    pub email: String,
}