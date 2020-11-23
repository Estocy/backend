use uuid::Uuid;

pub struct Client {
    id: Uuid,
    name: String,
    email: String,
    phone_number: String,
    address: String
}