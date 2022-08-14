pub(crate) mod movie;

use std::sync::Arc;

pub struct Models {
    pub movies: movie::MovieModel,
}

impl Models {
    pub fn new(db: Arc<sqlx::PgPool>) -> Self {
        Self {
            movies: movie::MovieModel { db },
        }
    }
}
