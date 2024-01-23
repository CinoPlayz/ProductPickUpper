#![warn(non_snake_case)]
use std::path::Path;
use owo_colors::{ colors::{ Green, Red }, OwoColorize };

use actix_web::{ get, App, HttpRequest, HttpServer, Responder };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut certExists: bool = false;
    let mut sslBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if Path::new("key.pem").exists() && Path::new("cert.pem").exists() {
        println!("Found TSL keys (key.pem, cert.pem)");
        certExists = true;
        // load TLS keys
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

        match sslBuilder.set_private_key_file("key.pem", SslFiletype::PEM) {
            Ok(_) => {
                println!("{}", format!("Opened key.pem").fg::<Green>());
            }
            Err(e) => {
                println!("{}", format!("Error while opening key.pem: {}", e).fg::<Red>());
                return Ok(());
            }
        }

        match sslBuilder.set_certificate_chain_file("cert.pem") {
            Ok(_) => {
                println!("{}", format!("Opened cert.pem").fg::<Green>());
            }
            Err(e) => {
                println!("{}", format!("Error while opening cert.pem: {}", e).fg::<Red>());
                return Ok(());
            }
        }
    } else {
        println!(
            "Did not found TSL keys (key.pem, cert.pem) {}",
            "Consider using TSL encryption for safer communication".yellow()
        );
    }

    let httpServer = HttpServer::new(|| App::new().service(index));

    if certExists {
        println!("{}", format!("Server starting").fg::<Green>());
        return httpServer
            .bind_openssl("127.0.0.1:8080", sslBuilder)
            .expect("Error while setting address with port/openssl: ")
            .run().await;
    } else {
        println!("{}", format!("Server starting").fg::<Green>());
        return httpServer
            .bind("127.0.0.1:8080")
            .expect("Error while setting address with port: ")
            .run().await;
    }
}
