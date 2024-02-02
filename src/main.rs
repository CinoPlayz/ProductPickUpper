#![allow(non_snake_case)]
use std::{ env, path::Path };
use owo_colors::{ colors::{ Green, Red }, OwoColorize };
use sqlx::MySqlPool;
use actix_web::{ get, web, App, HttpRequest, HttpServer, Responder };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
use crate::shared::{chrono::getCurrentTimeStr, password::createRoot, structs::structsApp::ApiDoc, structs::structsApp::AppState, structs::structsApp::HashingParameters};
mod handlers;
mod shared;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!();

    //Load environment variables
    dotenvy::from_path("config/.env").expect("Error while reding from config/.env: ");

    println!("{} - Reading MYSQL_URL from .env file", getCurrentTimeStr());
    let envDBUrl = env::var("MYSQL_URL").expect("Error while reding MYSQL_URL variable:");

    println!("{} - Reading IP_WITH_PORT from .env file", getCurrentTimeStr());
    let envIpWithPort = env
        ::var("IP_WITH_PORT")
        .expect("Error while reding IP_WITH_PORT variable:");

    println!("{} - Reading PASSWORD_PAPPER from .env file", getCurrentTimeStr());
    let envPasswordPapper = env
        ::var("IP_WITH_PORT")
        .expect("Error while reding IP_WITH_PORT variable:");

    println!("{} - Reading CREATE_ROOT from .env file", getCurrentTimeStr());
    let envCreateRoot = env
        ::var("CREATE_ROOT")
        .expect("Error while reding CREATE_ROOT variable:")
        .parse::<bool>().expect("Error while converting CREATE_ROOT to bool:");
   
    println!("{} - Reading MEM_COST from .env file", getCurrentTimeStr());
    let envMemCost = env
        ::var("MEM_COST")
        .expect("Error while reding MEM_COST variable:")
        .parse::<u32>().expect("Error while converting MEM_COST to u32:");

    println!("{} - Reading TIME_COST from .env file", getCurrentTimeStr());
    let envTimeCost = env
        ::var("TIME_COST")
        .expect("Error while reding TIME_COST variable:")
        .parse::<u32>().expect("Error while converting TIME_COST to u32:");

    println!("{} - Reading LANES from .env file", getCurrentTimeStr());
    let envLanes = env
        ::var("LANES")
        .expect("Error while reding LANES variable:")
        .parse::<u32>().expect("Error while converting envLanes to u32:");

    let hashingParameters = HashingParameters {
        mem_cost: envMemCost.clone(),
        time_cost: envTimeCost.clone(),
        lanes: envLanes.clone()
    };


    //Tries to connect to a database
    println!("{} - Connecting to databse", getCurrentTimeStr());
    let pool = MySqlPool::connect(&envDBUrl).await.expect(
        "Error while trying to connect to database"
    );
    println!("{} - {}", getCurrentTimeStr(), format!("Connected to databse").fg::<Green>());

    //Inserts root if CREATE_ROOT is true
    if envCreateRoot{
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            "Inserting root user (this might take some time)"
        );

        match createRoot(&pool, &envPasswordPapper, &hashingParameters).await  {
            Err(e) => {
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Error while inserting root user: {}", e.Message).fg::<Red>()
                );
                return Ok(());
            },
            Ok(_) => {
                println!(
                    "{} - {} {}",
                    getCurrentTimeStr(),
                    format!("Inserted root user (Password: admin)").fg::<Green>(),
                    format!("Use this environment variable only when seting up the server!!! (set it to false when done)").fg::<Red>()
                );
            }            
        }
            
        
    }


    //Tries to find TLS keys for secure communication
    println!("{} - Finding TLS keys", getCurrentTimeStr());
    let mut certExists: bool = false;
    let mut sslBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect(
        "Error while creating ssl builder: "
    );
    if Path::new("config/key.pem").exists() && Path::new("config/cert.pem").exists() {
        println!("{} - Found TLS keys (key.pem, cert.pem)", getCurrentTimeStr());
        certExists = true;
        // load TLS keys
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

        match sslBuilder.set_private_key_file("config/key.pem", SslFiletype::PEM) {
            Ok(_) => {
                println!("{} - {}", getCurrentTimeStr(), format!("Opened key.pem").fg::<Green>());
            }
            Err(e) => {
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Error while opening key.pem: {}", e).fg::<Red>()
                );
                return Ok(());
            }
        }

        match sslBuilder.set_certificate_chain_file("config/cert.pem") {
            Ok(_) => {
                println!("{} - {}", getCurrentTimeStr(), format!("Opened cert.pem").fg::<Green>());
            }
            Err(e) => {
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Error while opening cert.pem: {}", e).fg::<Red>()
                );
                return Ok(());
            }
        }
    } else {
        println!(
            "{} - Did not found TLS keys (key.pem, cert.pem) {}",
            getCurrentTimeStr(),
            "Consider using TLS encryption for safer communication".yellow()
        );
    }

    //TODO: Custom error reporting using PickUpErrorStruct
    let httpServer = HttpServer::new(move ||
        App::new()
            .app_data(
                web::Data::new(AppState {
                    version: VERSION.to_string(),
                    pepper: envPasswordPapper.clone(),
                    pool: pool.clone(),
                    createRoot: envCreateRoot.clone(),
                    hashingParameters: HashingParameters {
                        mem_cost: envMemCost.clone(),
                        time_cost: envTimeCost.clone(),
                        lanes: envLanes.clone()
                    }
                })
            )
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/docs/openapi.json", ApiDoc::openapi())
                    .config(Config::default().use_base_layout().filter(true)),
            )
            .service(index)
            .service(handlers::User::userGet::getAllUsers)
            .service(handlers::User::userPost::postUser)
            .service(handlers::Token::login::login)
    );

    if certExists {
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            format!("Server starting with {}", envIpWithPort).fg::<Green>()
        );
        return httpServer
            .bind_openssl(&envIpWithPort, sslBuilder)
            .expect("Error while setting address with port/openssl: ")
            .run().await;
    } else {
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            format!("Server starting with {}", envIpWithPort).fg::<Green>()
        );
        return httpServer
            .bind(&envIpWithPort)
            .expect("Error while setting address with port: ")
            .run().await;
    }
}
