use anyhow::Ok;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, query};

use crate::handler::user_handler::RegisterData;

#[derive(Serialize, Deserialize, Clone, Default, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn create(pool: &PgPool, data: RegisterData) -> anyhow::Result<Self> {
        let user: User =
            sqlx::query_as("INSERT INTO users (name, email) VALUES ($1, $2) Returning *")
                .bind(&data.name)
                .bind(&data.email)
                .fetch_one(pool)
                .await?;

        Ok(user)
    }

    pub async fn all(pool: &PgPool) -> anyhow::Result<Vec<User>> {
        let rows = query("SELECT * FROM users").fetch_all(pool).await?;

        let mut users = vec![];

        for row in rows {
            let user = User::from_row(&row).unwrap();
            users.push(user);
        }

        Ok(users)
    }

    pub async fn find(pool: &PgPool, id: i32) -> anyhow::Result<User> {
        let row = query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let user = User::from_row(&row).unwrap();

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update(pool: &PgPool, id: i32, data: RegisterData) -> anyhow::Result<()> {
        query("UPDATE users SET name = $1, email = $2 WHERE id = $3")
            .bind(&data.name)
            .bind(&data.email)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
