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
        code -> Int4,
        description -> Varchar,
        store_name -> Nullable<Varchar>,
        store_price -> Nullable<Float4>,
        price -> Float4,
        additional_charge -> Nullable<Float4>,
        color -> Varchar,
        weight -> Float4,
        brand -> Varchar,
        stock_amount -> Int4,
    }
}

table! {
    products_categories (id) {
        id -> Uuid,
        product_id -> Uuid,
        category_id -> Uuid,
    }
}

table! {
    requests (id) {
        id -> Uuid,
        user_id -> Uuid,
        client_id -> Uuid,
        sale_date -> Date,
        delivery_date -> Date,
        status -> Int4,
        comments -> Nullable<Varchar>,
        price -> Float4,
        discount -> Float4,
        freight -> Float4,
    }
}

table! {
    requests_products (id) {
        id -> Uuid,
        request_id -> Uuid,
        product_id -> Uuid,
        amount -> Int4,
        additional_costs -> Float4,
        discount -> Float4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        share_photos -> Nullable<Bool>,
        darkmode -> Nullable<Bool>,
    }
}

joinable!(products_categories -> categories (category_id));
joinable!(products_categories -> products (product_id));
joinable!(requests -> clients (client_id));
joinable!(requests -> users (user_id));
joinable!(requests_products -> products (product_id));
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
