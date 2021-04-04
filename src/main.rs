mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running server at http://127.0.0.1:8080");
    server::run().await
}
