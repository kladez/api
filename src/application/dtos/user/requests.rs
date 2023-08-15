use poem_openapi::{
    types::{
        Email,
        Password,
    },
    Object,
};

#[derive(Debug, Object, Clone, Eq, PartialEq)]
#[oai(rename = "CreateUserRequest")]
pub struct Create {
    /// Name
    #[oai(validator(max_length = 64))]
    pub name: String,
    /// Email
    pub email: Email,
    /// Password
    #[oai(validator(max_length = 32))]
    pub password: Password,
}
