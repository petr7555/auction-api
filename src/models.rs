use crate::schema::{auctions, bids, users};
use chrono::{DateTime, NaiveDateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};
use diesel::sql_types::Text;
use diesel::sql_types::BigInt;
use diesel::sql_types::Timestamptz;
use diesel::sql_types::Nullable;

#[derive(Identifiable, Serialize, Queryable, Getters)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    name: String,
    #[getset(get = "pub")]
    #[serde(skip)]
    password: String,
    active: bool,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Getters, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct NewUser {
    #[getset(get = "pub")]
    pub name: String,
    #[getset(get = "pub")]
    pub password: String,
}

#[derive(Identifiable, Serialize, Queryable, Associations)]
#[serde(rename_all = "camelCase")]
#[belongs_to(User)]
#[belongs_to(Auction)]
pub struct Bid {
    pub id: i64,
    pub user_id: i64,
    auction_id: i64,
    pub amount: i64,
    active: bool,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Getters)]
#[serde(rename_all = "camelCase")]
#[table_name = "bids"]
pub struct NewBid {
    #[getset(get = "pub")]
    pub user_id: i64,
    pub auction_id: i64,
    pub amount: i64,
}

#[derive(Identifiable, Serialize, Queryable, Associations, Getters)]
#[serde(rename_all = "camelCase")]
#[belongs_to(User)]
pub struct Auction {
    #[getset(get = "pub")]
    id: i64,
    user_id: i64,
    name: String,
    description: String,
    until: DateTime<Utc>,
    active: bool,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Getters)]
#[serde(rename_all = "camelCase")]
#[table_name = "auctions"]
pub struct NewAuction {
    pub name: String,
    pub description: String,
    pub until: DateTime<Utc>,
    #[getset(get = "pub")]
    pub user_id: i64,
}

#[derive(Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct ReturnAuction {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    name: String,
    #[sql_type = "Text"]
    description: String,
    #[sql_type = "Text"]
    user: String,
    #[sql_type = "Timestamptz"]
    until: DateTime<Utc>,
    #[sql_type = "Nullable<BigInt>"]
    actual_price: Option<i64>,
    #[sql_type = "Nullable<Text>"]
    winning_user: Option<String>,
}
