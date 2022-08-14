use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Movie {
    id: i64,
    title: String,
    year: i32,
    runtime: i32,
    genres: Vec<String>,
    #[serde(skip)]
    created_at: DateTime<Utc>,
    version: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct MovieParams {
    #[validate(
        required(message = "cannot be blank"),
        length(min = 1, message = "cannot be blank"),
        length(max = 500, message = "cannot be longer than 500 bytes")
    )]
    title: Option<String>,

    #[validate(
        required(message = "cannot be blank"),
        range(min = 1888, message = "cannot be less than 1888")
    )]
    year: Option<i32>,

    #[validate(
        required(message = "cannot be blank"),
        range(min = 1, message = "cannot be less than 1")
    )]
    runtime: Option<i32>,

    #[validate(
        required(message = "cannot be blank"),
        length(min = 1, message = "cannot be empty"),
        length(max = 5, message = "cannot have more than 5 genres")
    )]
    genres: Option<Vec<String>>,
}

pub struct MovieModel {
    pub(super) db: Arc<sqlx::PgPool>,
}

impl MovieModel {
    pub async fn get(&self, id: i64) -> Result<Movie, sqlx::Error> {
        sqlx::query_as::<_, Movie>(
            "SELECT id, title, year, runtime, genres, created_at, version
                FROM movies
                WHERE id = $1",
        )
        .bind(id)
        .fetch_one(&*self.db)
        .await
    }

    pub async fn insert(&self, params: &MovieParams) -> sqlx::Result<Movie> {
        sqlx::query_as::<_, Movie>(
            "INSERT INTO movies (title, year, runtime, genres)
                VALUES($1, $2, $3, $4)
                RETURNING *",
        )
        .bind(params.title.as_ref().unwrap())
        .bind(params.year.unwrap())
        .bind(params.runtime.unwrap())
        .bind(params.genres.as_ref().unwrap())
        .fetch_one(&*self.db)
        .await
    }
}
