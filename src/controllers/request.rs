use crate::database::connection::establish_connection;
use crate::database::schema::requests_products::dsl;
use crate::database::schema::requests::dsl::requests;
use crate::database::schema::requests_products::dsl::requests_products;
use crate::models::request::{Request, NewRequest, RequestReceiver};
use crate::models::request_product::{RequestProduct, NewRequestProduct};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format="json", data = "<request>")]
pub fn create(request: Json<RequestReceiver>) -> Json<Request> {
    let connection = establish_connection();

    let request_created:Request = diesel::insert_into(requests)
        .values(&request.0.request)
        .get_result(&connection)
        .unwrap();


    let product_category:RequestProduct = diesel::insert_into(requests_products)
        .values((
            dsl::request_id.eq(request_created.id), 
            dsl::product_id.eq(request.0.product_id), 
            dsl::amount.eq(request.0.amount), 
            dsl::additional_costs.eq(request.0.additional_costs), 
            dsl::discount.eq(request.0.discount)
        ))
        .get_result(&connection)
        .unwrap();


    Json(request_created)
}

#[get("/")]
pub fn index() -> Json<Vec<Request>> {
    let connection = establish_connection();
    let results = requests.load::<Request>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<Request>>{
    let connection = establish_connection();
    let results = requests.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Request>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        None
    } else {
        Some(Json(results[0].clone()))
    }
}

#[delete("/<id>")]
pub fn delete(id: String) -> Option<()>{
    let connection = establish_connection();
    let request = requests.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(request)
        .execute(&connection);

    match result {
        Ok(qnt_deleted) => {
            if qnt_deleted == 0 {
                None
            } else {
                Some(())
            }
        },
        Err(_) => None
    }
}