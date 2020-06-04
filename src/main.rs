#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, App, HttpServer};

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use diesel_migrations::embed_migrations;

mod actions;
mod get;
mod models;
mod post;
mod response;
mod schema;

embed_migrations!();

async fn initial_data(conn: &PgConnection) {
    if actions::find_user_by_name(conn, "Joe").unwrap().is_some() {
        println!("database is not empty, skipping data init");
        return;
    }

    let new_user = models::NewUser {
        name: "Joe".to_owned(),
        password: "1234".to_owned(),
    };
    let new_user_2 = models::NewUser {
        name: "Smith".to_owned(),
        password: "134861234".to_owned(),
    };
    let new_user_3 = models::NewUser {
        name: "Michael".to_owned(),
        password: "fsadfasdfsad".to_owned(),
    };
    let user = actions::insert_new_user(conn, &new_user).unwrap();
    let user2 = actions::insert_new_user(conn, &new_user_2).unwrap();
    let user3 = actions::insert_new_user(conn, &new_user_3).unwrap();
    let new_auction = models::NewAuction {
        name: "Brand new couch".to_owned(),
        description: "Too large for the living room".to_owned(),
        until: chrono::offset::Utc::now(),
        user_id: user.id,
    };
    let new_auction_2 = models::NewAuction {
        name: "Old fridge".to_owned(),
        description: "Need to get rid of it".to_owned(),
        until: chrono::offset::Utc::now() - chrono::Duration::days(2),
        user_id: user.id,
    };
    let new_auction_3 = models::NewAuction {
        name: "PS4".to_owned(),
        description: "Firmware 4.05".to_owned(),
        until: chrono::offset::Utc::now() + chrono::Duration::weeks(1),
        user_id: user2.id,
    };
    let new_auction_4 = models::NewAuction {
        name: "XB1".to_owned(),
        description: "Brand new".to_owned(),
        until: chrono::offset::Utc::now() - chrono::Duration::days(1),
        user_id: user2.id,
    };
    let _auction = actions::insert_new_auction(conn, &new_auction).unwrap();
    let _auction2 = actions::insert_new_auction(conn, &new_auction_2).unwrap();
    let auction3 = actions::insert_new_auction(conn, &new_auction_3).unwrap();
    let auction4 = actions::insert_new_auction(conn, &new_auction_4).unwrap();

    let new_bid = models::NewBid {
        user_id: user3.id,
        auction_id: *auction3.id(),
        amount: 200,
    };

    let new_bid_2 = models::NewBid {
        user_id: user3.id,
        auction_id: *auction4.id(),
        amount: 100,
    };

    let _bid = actions::insert_new_bid(conn, &new_bid).unwrap();
    let _bid2 = actions::insert_new_bid(conn, &new_bid_2).unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    {
        let conn = pool.get().expect("Database pool error");
        embedded_migrations::run(&conn).expect("Migration init failed");
        initial_data(&conn).await;
    }

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let bind = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                actix_cors::Cors::new()
                    .supports_credentials()
                    .finish()
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .service(get::user)
            .service(get::auction)
            .service(get::all_auctions)
            .service(get::all_auctions_user_created)
            .service(get::auctions_taken_part_user)
            .service(get::logout_user)
            .service(post::register_user)
            .service(post::login_user)
            .service(post::create_auction)
            .service(post::create_bid)
    })
    .bind(&bind)?
    .run()
    .await
    .and_then(|e| {
        println!("Server started");
        Ok(e)
    })
}
