use sqlx::{Error, MySql, Pool};
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use crate::user::{UserDAO, User};
use crate::utils::uid::generate_uid;

pub struct UserService {
    db: Pool<MySql>
}

impl UserService {
    pub fn new(db: Pool<MySql>) -> Self {
        UserService {
            db
        }
    }

    pub async fn register(&self, new_user: &mut UserDAO) -> Result<MySqlQueryResult, Error> {
        let user: User = new_user.as_user(generate_uid());

        let result = sqlx::query("INSERT INTO users VALUES(?, ?, ?)")
            .bind(user.id)
            .bind(user.email)
            .bind(user.password)
            .execute(&self.db)
            .await;

        return result;
    }

    pub async fn login(&self, email: &String, password: &String) -> Result<MySqlRow, Error> {
        let result = sqlx::query("SELECT (id) FROM users WHERE email=? AND pw_hash=?")
            .bind(email)
            .bind(password)
            .fetch_one(&self.db)
            .await;

        return result;
    }

    pub async fn change_password(&self, user_id: &String, new_password: &String) -> Result<MySqlQueryResult, Error> {
        let result = sqlx::query("UPDATE users SET pw_hash=? WHERE id=?")
            .bind(new_password)
            .bind(user_id)
            .execute(&self.db)
            .await;

        return result;
    }
}