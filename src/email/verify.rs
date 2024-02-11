use askama::Template;


#[derive(Template)]
#[template(path = "verify.html")]
pub struct VerifyEmailTemplate<'a> {
    pub app_name: &'a str,
    pub verify_link: &'a str,
}
