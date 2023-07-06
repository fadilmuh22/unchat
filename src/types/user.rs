#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub avatar: String,
    pub username: String,

    pub created_at: String,
    pub updated_at: String,
}