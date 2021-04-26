table! {
    items (id) {
        id -> Integer,
        item_id -> Varchar,
        name -> Varchar,
        picture -> Nullable<Varchar>,
        price -> Integer,
        stock -> Integer,
        owner -> Varchar,
        posted_timestamp -> Datetime,
    }
}
