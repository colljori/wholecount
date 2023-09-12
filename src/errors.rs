/* Copyright (C) 2023 COLLOMB Joris - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the XYZ license, which unfortunately won't be
 * written for another century.
 *
 * You should have received a copy of the XYZ license with
 * this file. If not, please write to: joris.collomb@gmail.com.
 */

// DEPENDENCIES  ---------------------------------------------------------------------------------
//

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

// TYPE AND CONSTANT -----------------------------------------------------------------------------
//

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

// PUBLIC SERVICES -------------------------------------------------------------------------------
//

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
