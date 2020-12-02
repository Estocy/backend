use crate::database::connection::establish_connection;
use crate::database::schema::categories::dsl::categories;
use crate::database::schema::products::dsl::products;
use crate::database::schema::products_categories::dsl;
use crate::database::schema::products_categories::dsl::products_categories;
use crate::models::category::Category;
use crate::models::product::{Product, NewProductReceiver, ProductReceiver};
use crate::models::product_category::ProductCategory;
use diesel::prelude::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format = "json", data = "<product>")]
pub fn create(product: Json<NewProductReceiver>) -> Option<Json<Product>> {
    let connection = establish_connection();

    use crate::database::schema::products::dsl;

    let results = products
        .filter(dsl::code.eq(&product.0.product.code))
        .load::<Product>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        use crate::database::schema::products_categories::dsl;

        let product_created: Product = diesel::insert_into(products)
            .values(&product.0.product)
            .get_result(&connection)
            .unwrap();

        for category in &product.0.categories {
            let _product_category: ProductCategory = diesel::insert_into(products_categories)
                .values((
                    dsl::product_id.eq(product_created.id),
                    dsl::category_id.eq(category),
                ))
                .get_result(&connection)
                .unwrap();
        }

        Some(Json(product_created))
    }
    else {
        None
    }
}

#[put("/", format = "json", data = "<product>")]
pub fn update(product: Json<ProductReceiver>) -> Json<Product> {
    let connection = establish_connection();

    use crate::database::schema::products_categories::dsl;

    let product_updated: Product = diesel::update(&product.0.product)
        .set(&product.0.product)
        .get_result(&connection)
        .unwrap();

    let _affected = diesel::delete(products_categories.filter(dsl::product_id.eq(product_updated.id)))
        .execute(&connection)
        .unwrap();

    for category in &product.0.categories {
        let _product_category: ProductCategory = diesel::insert_into(products_categories)
            .values((
                dsl::product_id.eq(product_updated.id),
                dsl::category_id.eq(category),
            ))
            .get_result(&connection)
            .unwrap();
    }

    Json(product_updated)
}

#[get("/")]
pub fn index() -> Json<Vec<(Product, Vec<Category>)>> {
    let connection = establish_connection();

    let results = products
        .load::<Product>(&connection)
        .expect("Error loading products");

    let mut list_products: Vec<(Product, Vec<Category>)> = Vec::new();

    for product in results {
        let results_product_category = products_categories
            .filter(dsl::product_id.eq(product.id))
            .load::<ProductCategory>(&connection)
            .expect("Error loading posts");

        let mut list_categories = Vec::new();

        for product_category in results_product_category {
            let results_product_category = categories
                .find(product_category.category_id)
                .load::<Category>(&connection)
                .expect("Error loading posts");

            list_categories.push(results_product_category[0].clone());
        }

        list_products.push((product.clone(), list_categories.clone()))
    }

    Json(list_products)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<(Product, Vec<Category>)>> {
    let connection = establish_connection();

    let results = products
        .find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Product>(&connection)
        .expect("Error loading Producst");

    if results.len() != 0 {
        let results_product_category = products_categories
            .filter(dsl::product_id.eq(Uuid::parse_str(id.as_str()).unwrap()))
            .load::<ProductCategory>(&connection)
            .expect("Error loading product categories");

        let mut list_categories = Vec::<Category>::new();

        for product_category in results_product_category {
            let results_product_category = categories
                .find(product_category.category_id)
                .load::<Category>(&connection)
                .expect("Error loading categories");

            list_categories.push(results_product_category[0].clone());
        }

        Some(Json((results[0].clone(), list_categories.clone())))
    }
    else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: String) -> Option<()> {
    let connection = establish_connection();

    let product = products.find(Uuid::parse_str(id.as_str()).unwrap());

    let result = diesel::delete(product).execute(&connection);

    match result {
        Ok(qnt_deleted) => {
            if qnt_deleted == 0 {
                None
            }
            else {
                Some(())
            }
        }
        Err(_) => None,
    }
}
