use diesel::table;

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
    }
}

