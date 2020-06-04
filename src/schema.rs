table! {
    auctions (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        description -> Varchar,
        until -> Timestamptz,
        active -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    bids (id) {
        id -> Int8,
        user_id -> Int8,
        auction_id -> Int8,
        amount -> Int8,
        active -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        password -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(auctions -> users (user_id));
joinable!(bids -> auctions (auction_id));
joinable!(bids -> users (user_id));

allow_tables_to_appear_in_same_query!(
    auctions,
    bids,
    users,
);
