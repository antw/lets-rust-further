use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub year: i32,
    pub runtime: i32,
    pub genres: Vec<String>,
    #[serde(skip)]
    pub created_at: DateTime<Utc>,
    pub version: i32,
}

impl Movie {
    pub fn update(&mut self, params: MovieParams) {
        if params.title.is_some() {
            self.title = params.title.unwrap();
        }

        if params.year.is_some() {
            self.year = params.year.unwrap();
        }

        if params.runtime.is_some() {
            self.runtime = params.runtime.unwrap();
        }

        if params.genres.is_some() {
            self.genres = params.genres.unwrap();
        }
    }
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
        sqlx::query_as!(
            Movie,
            "SELECT id, title, year, runtime, genres, created_at, version
                FROM movies
                WHERE id = $1",
            id
        )
        .fetch_one(&*self.db)
        .await
    }

    pub async fn insert(&self, params: &MovieParams) -> sqlx::Result<Movie> {
        sqlx::query_as!(
            Movie,
            "INSERT INTO movies (title, year, runtime, genres)
                VALUES($1, $2, $3, $4)
                RETURNING *",
            params.title.as_ref().unwrap(),
            params.year.unwrap(),
            params.runtime.unwrap(),
            params.genres.as_ref().unwrap()
        )
        .fetch_one(&*self.db)
        .await
    }

    pub async fn update(&self, movie: &mut Movie) -> sqlx::Result<()> {
        let result = sqlx::query!(
            "UPDATE movies
                SET title = $1, year = $2, runtime = $3, genres = $4, version = version + 1
                WHERE id = $5
                RETURNING version",
            &movie.title,
            &movie.year,
            &movie.runtime,
            &movie.genres,
            &movie.id
        )
        .fetch_one(&*self.db)
        .await?;

        movie.version = result.version;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM movies WHERE id = $1", id)
            .execute(&*self.db)
            .await?;

        Ok(())
    }
}
