use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    share_photos: bool,
    darkmode: bool
}