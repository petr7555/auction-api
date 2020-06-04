use crate::models::{Auction, ReturnAuction, Bid, NewAuction, NewBid, NewUser, User};
use diesel::prelude::*;
use diesel::sql_types::BigInt;

const QUERY_ALL_AUCTIONS: &str = " \
    SELECT DISTINCT auctions.id, auctions.name, auctions.description, users.name as user, auctions.until, COALESCE(bids.amount, 0) AS actual_price, users2.name AS winning_user \
    FROM \
        auctions \
        INNER JOIN users \
            ON auctions.user_id = users.id \
        LEFT OUTER JOIN \
            (SELECT DISTINCT ON (auction_id) user_id, auction_id, amount FROM bids ORDER BY auction_id, amount DESC) AS bids \
            ON auctions.id = bids.auction_id \
        LEFT OUTER JOIN users users2 \
            ON bids.user_id = users2.id";

pub fn find_user_by_id(
    conn: &PgConnection,
    user_id: i64,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_auction_by_id(
    conn: &PgConnection,
    auction_id: i64,
) -> Result<Option<ReturnAuction>, diesel::result::Error> {

    let auction = diesel::sql_query(format!("{} WHERE auctions.id= $1 LIMIT 1", QUERY_ALL_AUCTIONS))
        .bind::<BigInt, _>(auction_id)
        .get_result::<ReturnAuction>(conn)
        .optional()?;

    Ok(auction)
}

pub fn find_user_by_name(
    conn: &PgConnection,
    user_name: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(name.eq(user_name))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    conn: &PgConnection,
    new_user: &NewUser,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    //use diesel::dsl::select;
    //use diesel::dsl::exists;
    //let user_exists = select(exists(users.filter(name.eq(&new_user.name)))).get_result(conn);
    //if Ok(true) == user_exists {
    //   return  Err(diesel::result::Error::CustomError);
    //}

    let inserted_user = diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(conn)?;

    Ok(inserted_user)
}

pub fn find_all_auctions(conn: &PgConnection) -> Result<Vec<ReturnAuction>, diesel::result::Error> {

    let auction_vec = diesel::sql_query(QUERY_ALL_AUCTIONS)
        .load::<ReturnAuction>(conn)?;

    Ok(auction_vec)
}

pub fn insert_new_auction(
    conn: &PgConnection,
    new_auction: &NewAuction,
) -> Result<Auction, diesel::result::Error> {
    use crate::schema::auctions::dsl::*;

    let inserted_auction = diesel::insert_into(auctions)
        .values(new_auction)
        .get_result::<Auction>(conn)?;

    Ok(inserted_auction)
}

pub fn find_auctions_by_user_id(
    conn: &PgConnection,
    user_id: i64,
) -> Result<Vec<ReturnAuction>, diesel::result::Error> {

    let auction_vec = diesel::sql_query(format!("{} WHERE users.id= $1", QUERY_ALL_AUCTIONS))
        .bind::<BigInt, _>(user_id)
        .load::<ReturnAuction>(conn)?;

    Ok(auction_vec)
}

pub fn insert_new_bid(conn: &PgConnection, new_bid: &NewBid) -> Result<Bid, diesel::result::Error> {
    use crate::schema::*;

    let highest_bid_res = bids::dsl::bids
        .filter(bids::auction_id.eq(new_bid.auction_id))
        .order((bids::amount.desc(), bids::created_at))
        .first::<Bid>(conn)
        .optional()?;

    if highest_bid_res.is_some() {
        let highest_bid = highest_bid_res.unwrap();
        if highest_bid.amount >= new_bid.amount {
            return Ok(highest_bid)
        }
    }

    let inserted_bid = diesel::insert_into(bids::dsl::bids)
        .values(new_bid)
        .get_result::<Bid>(conn)?;

    Ok(inserted_bid)
}

pub fn find_auctions_user_taken_part_in(
    conn: &PgConnection,
    searched_user_id: i64,
) -> Result<Vec<ReturnAuction>, diesel::result::Error> {
    
    let auction_vec = diesel::sql_query(format!("{} INNER JOIN bids bids2 ON auctions.id = bids2.auction_id WHERE bids2.user_id= $1", QUERY_ALL_AUCTIONS))
        .bind::<BigInt, _>(searched_user_id)
        .load::<ReturnAuction>(conn)?;
    
    Ok(auction_vec)
}
