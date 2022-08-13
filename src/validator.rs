use std::collections::HashMap;
use std::hash::Hash;

use axum::{http::StatusCode, response::IntoResponse, Json};
use regex::Regex;

lazy_static! {
    static ref EMAIL_RX: Regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+\\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
}

#[derive(Debug)]
pub(crate) struct Validator<'a> {
    errors: HashMap<&'a str, &'a str>,
}

impl<'a> Validator<'a> {
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, key: &'a str, message: &'a str) {
        self.errors.entry(key).or_insert(message);
    }

    pub fn check(&mut self, ok: bool, key: &'a str, message: &'a str) {
        if !ok {
            self.add_error(key, message);
        }
    }
}

impl<'a> IntoResponse for Validator<'a> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNPROCESSABLE_ENTITY, Json(self.errors)).into_response()
    }
}

pub(crate) fn permitted_value<T>(value: T, permitted: Vec<T>) -> bool
where
    T: PartialEq<T>,
{
    if permitted.iter().any(|item| *item == value) {
        return true;
    }

    return false;
}

pub(crate) fn matches(value: String, rx: Regex) -> bool {
    rx.is_match(&value)
}

pub(crate) fn unique<T>(values: &[T]) -> bool
where
    T: Eq + Hash + std::fmt::Debug,
{
    let mut map = HashMap::new();

    for value in values.iter() {
        if map.contains_key(value) {
            return false;
        }

        map.insert(value, true);
    }

    return true;
}
