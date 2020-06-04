use crate::models::{NewAuction, NewUser, NewBid};
use crate::response::{SuccessResponse, ErrorResponse};
use actix_web::{error::BlockingError, post, web, HttpResponse};
use actix_identity::Identity;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{r2d2, PgConnection};

use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

type PoolConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn get_conn(pool: web::Data<DbPool>) -> Result<PoolConnection, HttpResponse> {
    pool.get().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })
}

enum Response{
    OK,
    DUPLICATE,
    ERROR
}

fn process_db_res<T>(res: Result<T, BlockingError<diesel::result::Error>>) -> Response
where
    T: Send + 'static
{
    match res {
        Ok(_) => Response::OK,
        Err(err) => match err {
            BlockingError::Error(diesel_error) => match diesel_error {
                DatabaseError(UniqueViolation, _msg) => Response::DUPLICATE,
                _ => Response::ERROR,
            },
            BlockingError::Canceled => Response::ERROR,
        },
    }
}

#[post("/api/register")]
pub async fn register_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = get_conn(pool)?;

    let new_user = form.into_inner();

    let res = web::block(move || actions::insert_new_user(&conn, &new_user)).await;
    match process_db_res(res) {
        Response::OK => 
            Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Response::DUPLICATE => 
            Ok(HttpResponse::Conflict().json(SuccessResponse { success: false })),
        Response::ERROR => 
            Err(HttpResponse::InternalServerError().finish().into()),
    }
}

#[post("/api/login")]
pub async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
    id: Identity,
) -> Result<HttpResponse, actix_web::Error> {

    let conn = get_conn(pool)?;

    let user = form.into_inner();
    let user_moved = user.clone();
    let res = web::block(move || actions::find_user_by_name(&conn, user_moved.name())).await;
    if let Ok(user_res) = res {
        if let Some(user_res) = user_res {
            if user.password() == user_res.password() {
                id.remember(user_res.id.to_string());
                return Ok(HttpResponse::Ok().json(user_res))
            }
        }
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse::from("unknown user or invalid password".to_owned())))
    }
    return Err(HttpResponse::InternalServerError().finish().into())
}

fn identity_check(id: Identity, user_id: i64) -> Result<(), HttpResponse> {
    if id.identity().is_none() {
        return Err(HttpResponse::Unauthorized().json(ErrorResponse::from("Authorization required".to_owned())))
    }
    if id.identity().unwrap() != user_id.to_string() {
        return Err(HttpResponse::Unauthorized().json(ErrorResponse::from("Cannot make action for other user".to_owned())))
    }
    Ok(())
}

#[post("/api/auctions")]
pub async fn create_auction(
    pool: web::Data<DbPool>,
    form: web::Json<NewAuction>,
    id: Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let auction = form.into_inner();

    identity_check(id, auction.user_id)?;

    let conn = get_conn(pool)?;

    let res = web::block(move || actions::insert_new_auction(&conn, &auction)).await;

    match process_db_res(res) {
        Response::OK => 
            Ok(HttpResponse::Ok().json(SuccessResponse { success: true })),
        Response::DUPLICATE => 
            Ok(HttpResponse::Conflict().json(SuccessResponse { success: false })),
        Response::ERROR => 
            Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[post("/api/bids")]
pub async fn create_bid(
    pool: web::Data<DbPool>,
    form: web::Json<NewBid>,
    id: Identity,
) -> Result<HttpResponse, actix_web::Error> {
    let bid = form.into_inner();

    identity_check(id, bid.user_id)?;

    let conn = get_conn(pool)?;

    let user = bid.user_id;
    let amount = bid.amount;

    let res = web::block(move || actions::insert_new_bid(&conn, &bid)).await;

    match res {
        Ok(res_bid) => 
            if res_bid.amount == amount && res_bid.user_id == user {
                Ok(HttpResponse::Ok().json(SuccessResponse { success: true }))
            }
            else {
                Ok(HttpResponse::Conflict().json(SuccessResponse { success: false }))
            }
        Err(err) => match err {
            BlockingError::Error(diesel_error) => match diesel_error {
                DatabaseError(UniqueViolation, _msg) => Ok(HttpResponse::Conflict().json(SuccessResponse { success: false })),
                _ => Ok(HttpResponse::InternalServerError().finish()),
            },
            BlockingError::Canceled => Ok(HttpResponse::InternalServerError().finish()),
        },
    }
}
