use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub cpf: String,
  pub password: String,
  pub age: i32,
  pub isActive: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPartial {
  pub name: String,
  pub email: String,
  pub cpf: String,
  pub password: String,
  pub age: i32,
}
