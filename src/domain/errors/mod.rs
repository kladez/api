use argon2::password_hash::Error as Argon2Error;
use poem_openapi::payload::Json;

use crate::{
    application::dtos,
    infrastructure,
};

impl From<sqlx::Error> for dtos::Error {
    fn from(e: sqlx::Error) -> Self {
        let message = e.to_string();
        match &e {
            // Error returned from the database.
            sqlx::Error::Database(_) => {
                if message.ends_with(
                    ": duplicate key value violates unique constraint \"users_name_key\"",
                ) {
                    Self::BadRequest(Json("Name is already registered".to_string().into()))
                } else if message.ends_with(
                    ": duplicate key value violates unique constraint \"users_email_key\"",
                ) {
                    Self::BadRequest(Json("email is already registered".to_string().into()))
                } else {
                    Self::BadRequest(Json(message.into()))
                }
            }

            // No rows returned by a query that expected to return at least one row.
            sqlx::Error::RowNotFound => Self::NotFound(Json(message.into())),

            _ => Self::InternalServerError(Json(message.into())),
        }
    }
}

impl From<Argon2Error> for dtos::Error {
    fn from(e: Argon2Error) -> Self {
        let message = e.to_string();
        match e {
            // Invalid password
            Argon2Error::Password => Self::Unauthorized(Json(message.into())),

            // Argon2Error::PhcStringField: password hash string invalid
            // Argon2Error::Algorithm: unsupported algorithm
            // Argon2Error::B64Encoding(B64Error): "B64" encoding error
            // Argon2Error::Crypto: cryptographic error
            // Argon2Error::OutputSize: output size unexpected
            // Argon2Error::ParamNameDuplicated: duplicate parameter name encountered
            // Argon2Error::ParamNameInvalid: invalid parameter name
            // Argon2Error::ParamValueInvalid(InvalidValue): invalid parameter value
            // Argon2Error::ParamsMaxExceeded: maximum number of parameters exceeded
            // Argon2Error::PhcStringTrailingData: password hash string contains trailing data
            // Argon2Error::SaltInvalid(InvalidValue): salt invalid
            // Argon2Error::Version: invalid algorithm version
            _ => Self::InternalServerError(Json(message.into())),
        }
    }
}

impl From<infrastructure::kafka::Error> for dtos::Error {
    fn from(e: infrastructure::kafka::Error) -> Self {
        // TODO: add matching error
        Self::InternalServerError(Json(e.to_string().into()))
    }
}

impl From<futures_channel::oneshot::Canceled> for dtos::Error {
    fn from(e: futures_channel::oneshot::Canceled) -> Self {
        Self::InternalServerError(Json(e.to_string().into()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("kafka error: {0}")]
    KafkaError(#[from] infrastructure::kafka::Error),
}

impl From<Error> for dtos::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::DatabaseError(e) => e.into(),
            Error::KafkaError(e) => e.into(),
        }
    }
}
