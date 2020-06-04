use crate::response::ErrorResponse;
use actix_web::{get, web, HttpResponse};
use actix_identity::Identity;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use diesel::{r2d2, PgConnection};
use serde::Serialize;

use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_res_handler<T: Serialize>(
    db_response: Option<T>,
    err_str: String,
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(db_response) = db_response {
        Ok(HttpResponse::Ok().json(db_response))
    } else {
        let error = ErrorResponse::from(err_str);
        let res = HttpResponse::NotFound().json(error);
        Ok(res)
    }
}

type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_conn(pool: web::Data<DbPool>) -> Result<PoolConnection, HttpResponse> {
    pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })
}

#[get("/api/users/{id}")]
pub async fn user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;
    let user_id = user_id.into_inner();
    let user = web::block(move || actions::find_user_by_id(&conn, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    db_res_handler(user, format!("No user found with id: {}", user_id))
}

#[get("/api/auctions/{id}")]
pub async fn auction(
    pool: web::Data<DbPool>,
    auction_id: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let auction_id = auction_id.into_inner();

    let auction = web::block(move || actions::find_auction_by_id(&conn, auction_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    db_res_handler(auction, format!("No auction found with id: {}", auction_id))
}

#[get("/api/auctions")]
pub async fn all_auctions(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let auctions = web::block(move || actions::find_all_auctions(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().json(auctions))
}

#[get("/api/auctions/user/{id}")]
pub async fn all_auctions_user_created(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let user_id = user_id.into_inner();

    let auctions = web::block(move || actions::find_auctions_by_user_id(&conn, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().json(auctions))
}

#[get("/api/auctions-taken-part/user/{id}")]
pub async fn auctions_taken_part_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let user_id = user_id.into_inner();

    let auctions = web::block(move || actions::find_auctions_user_taken_part_in(&conn, user_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().json(auctions))
}

#[get("/api/logout")]
pub async fn logout_user(id: Identity) -> Result<HttpResponse, actix_web::Error> {
    println!("Forgetting: {}", id.identity().unwrap_or("None".into()));
    id.forget();
    Ok(HttpResponse::Ok().finish())
}