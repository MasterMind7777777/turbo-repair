// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Uuid,
        repair_shop_id -> Uuid,
        street -> Varchar,
        city -> Varchar,
        state -> Varchar,
        zip -> Varchar,
        country -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    bids (id) {
        id -> Uuid,
        repair_request_id -> Uuid,
        repair_shop_id -> Uuid,
        bid_amount -> Float8,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Uuid,
        repair_request_id -> Uuid,
        repair_shop_id -> Uuid,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repair_requests (id) {
        id -> Uuid,
        customer_id -> Uuid,
        description -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    repair_shops (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    staff (id) {
        id -> Uuid,
        user_id -> Uuid,
        repair_shop_id -> Uuid,
        role -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    status_pipeline (id) {
        id -> Uuid,
        order_id -> Uuid,
        status -> Varchar,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(addresses -> repair_shops (repair_shop_id));
diesel::joinable!(bids -> repair_requests (repair_request_id));
diesel::joinable!(bids -> repair_shops (repair_shop_id));
diesel::joinable!(orders -> repair_requests (repair_request_id));
diesel::joinable!(orders -> repair_shops (repair_shop_id));
diesel::joinable!(repair_requests -> users (customer_id));
diesel::joinable!(staff -> repair_shops (repair_shop_id));
diesel::joinable!(staff -> users (user_id));
diesel::joinable!(status_pipeline -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    bids,
    orders,
    repair_requests,
    repair_shops,
    staff,
    status_pipeline,
    users,
);
