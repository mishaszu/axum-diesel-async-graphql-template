use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: Uuid,
    pub user_email: String,
    pub is_admin: bool,
}

impl Ctx {
    pub fn new(user_id: Uuid, user_email: String, is_admin: bool) -> Self {
        Self {
            user_id,
            user_email,
            is_admin,
        }
    }
}
