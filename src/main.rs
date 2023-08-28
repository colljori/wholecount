/* Copyright (C) 2023 COLLOMB Joris - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the XYZ license, which unfortunately won't be
 * written for another century.
 *
 * You should have received a copy of the XYZ license with
 * this file. If not, please write to: joris.collomb@gmail.com.
 */

//! DEPENDENCIES  ---------------------------------------------------------------------------------
//!


#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod handlers;
mod models;
mod schema;


//! TYPE AND CONSTANT -----------------------------------------------------------------------------
//!

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


//! PUBLIC SERVICES -------------------------------------------------------------------------------
//!


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


//! PRIVATE SERVICES ------------------------------------------------------------------------------
//!