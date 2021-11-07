use actix_web::Responder;

pub async fn get_technologies() -> impl Responder {
    format!("hello from get technologies")
}

pub async fn get_technology_by_id() -> impl Responder {
    format!("hello from get technologies by id")
}

pub async fn add_technology() -> impl Responder {
    format!("hello from add technology")
}

pub async fn delete_technology() -> impl Responder {
    format!("hello from delete technology")
}