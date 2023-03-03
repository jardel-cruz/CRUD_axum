use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub age: Option<u32>,
    pub email: Option<String>,
    pub password: Option<String>,
}
