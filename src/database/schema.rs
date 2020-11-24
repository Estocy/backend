table! {
    categories (id) {
        id -> Uuid,
        label -> Varchar,
        tag_color -> Varchar,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
    }
}

table! {
    products (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Varchar,
        store_name -> Nullable<Varchar>,
        store_price -> Nullable<Numeric>,
        price -> Numeric,
        additional_charge -> Nullable<Numeric>,
        color -> Varchar,
        weight -> Numeric,
        brand -> Varchar,
        stock_amount -> Int4,
    }
}

table! {
    products_categories (id) {
        id -> Uuid,
        product_id -> Nullable<Uuid>,
        category_id -> Nullable<Uuid>,
        label -> Varchar,
        tag_color -> Varchar,
    }
}

table! {
    requests (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        client_id -> Nullable<Uuid>,
        sale_date -> Date,
        delivery_date -> Date,
        status -> Int4,
        comments -> Nullable<Varchar>,
        price -> Numeric,
        discount -> Numeric,
        freight -> Numeric,
    }
}

table! {
    requests_products (id) {
        id -> Uuid,
        request_id -> Nullable<Uuid>,
        product_id -> Nullable<Uuid>,
        amount -> Int4,
        additional_costs -> Numeric,
        discount -> Numeric,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        share_photos -> Bool,
        darkmode -> Bool,
    }
}

joinable!(products_categories -> categories (category_id));
joinable!(products_categories -> products (product_id));
joinable!(requests -> clients (client_id));
joinable!(requests -> users (user_id));
joinable!(requests_products -> clients (product_id));
joinable!(requests_products -> requests (request_id));

allow_tables_to_appear_in_same_query!(
    categories,
    clients,
    products,
    products_categories,
    requests,
    requests_products,
    users,
);
