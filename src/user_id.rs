use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserId {
    pub user_id: u32,
}
