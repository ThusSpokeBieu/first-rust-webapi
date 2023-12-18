use ntex::web;

#[utoipa::path(
  get,
  path = "/users",
  responses(
    (status = 200, description = "List of users", body = [User]),
  ),
)]
#[web::get("/users")]
pub async fn get_users() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  post,
  path = "/users",
  request_body = UserPartial,
  responses(
    (status = 201, description = "User created", body = User),
  ),
)]
#[web::post("/users")]
pub async fn create_user() -> web::HttpResponse {
  web::HttpResponse::Created().finish()
}

#[utoipa::path(
  get,
  path = "/users/{id}",
  responses(
    (status = 200, description = "User found", body = User),
    (status = 400, description = "User not found", body = HttpError)
  ),
)]
#[web::get("/users/{id}")]
pub async fn get_user() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  put,
  path = "/users/{id}",
  request_body = UserPartial,
  responses(
    (status = 200, description = "User updated", body = User),
    (status = 400, description = "User not found", body = HttpError)
  ),
)]
#[web::put("/users/{id}")]
pub async fn update_user() -> web::HttpResponse {
  web::HttpResponse::Ok().finish()
}

#[utoipa::path(
  delete,
  path = "/users/{id}",
  responses(
    (status = 200, description = "User deleted", body = User),
    (status = 400, description = "User not found", body = HttpError)
  ),
)]
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
