pub mod configuration;
pub mod routes;
pub mod startup;
// pub use routes::{health_check,subscribe};
// pub fn run(address: &str) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/health_check", web::get().to(health_check))
//             .route("/{name}", web::get().to(greet))
//     })
//     .bind(address)?
//     .run();
//     Ok(server)
// }
