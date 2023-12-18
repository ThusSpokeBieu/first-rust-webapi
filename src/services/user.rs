use ntex::web;

#[web::get("/users")]
pub async fn get_users() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::post("/users")]
pub async fn create_user() -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

#[web::get("/users/{id}")]
pub async fn get_user() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::put("/users/{id}")]
pub async fn update_user() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[web::delete("/users/{id}")]
pub async fn delete_user() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
  cfg.service(get_users);
  cfg.service(create_user);
  cfg.service(get_user);
  cfg.service(update_user);
  cfg.service(delete_user);
}
