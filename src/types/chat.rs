use super::user::User;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chat {
    pub uuid: String,
    pub content: String,
    pub user: User,

    pub user_id: String,
    pub to_user_id: String,

    pub created_at: String,
    pub updated_at: String,
}
