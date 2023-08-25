use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserCredentials {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDAO {
    pub email: String,
    pub password: String
}

impl UserDAO {
    pub fn as_user(&mut self, id: String) -> User {
        return User {
            id,
            email: std::mem::replace(&mut self.email, String::new()),
            password: std::mem::replace(&mut self.password, String::new())
        };
    }
}