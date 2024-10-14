use chapter3::configuration::get_configuration;
use chapter3::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let addr="127.0.0.1:8000";
    // run(addr)?.await
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    //连接数据库
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind  random port");
    run(listener, connection_pool)?.await
}
