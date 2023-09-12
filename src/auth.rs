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

use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

// TYPE AND CONSTANT -----------------------------------------------------------------------------
//

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// PUBLIC SERVICES -------------------------------------------------------------------------------
//

// Validate token
pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

// PRIVATE SERVICES ------------------------------------------------------------------------------
//

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}
