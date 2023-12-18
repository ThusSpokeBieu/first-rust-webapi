use ntex::web;

mod services;
mod error;
mod models;

#[web::get("/")]
async fn index() -> &'static str {
  "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
  web::server(|| {
    web::App::new()
      .configure(services::user::ntex_config)
      .default_service(web::route().to(services::default))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;

  Ok(())
}
