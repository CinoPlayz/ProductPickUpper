#![allow(non_snake_case)]
use crate::shared::{
    chrono::getCurrentTimeStr,
    password::createRoot
};
use crate::models::structsApp:: {AppState, HashingParameters, ApiDoc, PickUpError, PickUpErrorCode};
use actix_web::{
    error, get, http::header, web::{self, JsonConfig, PathConfig}, App, HttpRequest, HttpResponse, HttpServer, Responder
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use owo_colors::{
    colors::{Green, Red},
    OwoColorize,
};
use sqlx::MySqlPool;
use std::{env, path::Path};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
mod controllers;
mod shared;
mod models;
use actix_cors::Cors;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[get("/version")]
pub async fn version(data: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body(data.version.clone())    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!();

    //Load environment variables
    dotenvy::from_filename("config/.env").expect("Error while reding from config/.env: ");

    println!("{} - Reading MYSQL_URL from .env file", getCurrentTimeStr());
    let envDBUrl = env::var("MYSQL_URL").expect("Error while reding MYSQL_URL variable:");

    println!(
        "{} - Reading IP_WITH_PORT from .env file",
        getCurrentTimeStr()
    );
    let envIpWithPort =
        env::var("IP_WITH_PORT").expect("Error while reding IP_WITH_PORT variable:");

    println!(
        "{} - Reading PASSWORD_PAPPER from .env file",
        getCurrentTimeStr()
    );
    let envPasswordPapper =
        env::var("IP_WITH_PORT").expect("Error while reding IP_WITH_PORT variable:");

    println!(
        "{} - Reading CREATE_ROOT from .env file",
        getCurrentTimeStr()
    );
    let envCreateRoot = env::var("CREATE_ROOT")
        .expect("Error while reding CREATE_ROOT variable:")
        .parse::<bool>()
        .expect("Error while converting CREATE_ROOT to bool:");

    println!("{} - Reading MEM_COST from .env file", getCurrentTimeStr());
    let envMemCost = env::var("MEM_COST")
        .expect("Error while reding MEM_COST variable:")
        .parse::<u32>()
        .expect("Error while converting MEM_COST to u32:");

    println!("{} - Reading TIME_COST from .env file", getCurrentTimeStr());
    let envTimeCost = env::var("TIME_COST")
        .expect("Error while reding TIME_COST variable:")
        .parse::<u32>()
        .expect("Error while converting TIME_COST to u32:");

    println!("{} - Reading LANES from .env file", getCurrentTimeStr());
    let envLanes = env::var("LANES")
        .expect("Error while reding LANES variable:")
        .parse::<u32>()
        .expect("Error while converting envLanes to u32:");

    let hashingParameters = HashingParameters {
        mem_cost: envMemCost.clone(),
        time_cost: envTimeCost.clone(),
        lanes: envLanes.clone(),
    };

    //Tries to connect to a database
    println!("{} - Connecting to databse", getCurrentTimeStr());
    let pool = MySqlPool::connect(&envDBUrl)
        .await
        .expect("Error while trying to connect to database");
    println!(
        "{} - {}",
        getCurrentTimeStr(),
        format!("Connected to databse").fg::<Green>()
    );

    //Change mysql session timezone to UTC
    println!("{} - Changing mysql session timezone", getCurrentTimeStr());
    sqlx::query!(
        "SET SESSION time_zone = '+00:00';",
    )
    .execute(&pool)
    .await
    .expect("Error while trying to set timezone");

    //Inserts root if CREATE_ROOT is true
    if envCreateRoot {
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            "Inserting root user (this might take some time)"
        );

        match createRoot(&pool, &envPasswordPapper, &hashingParameters).await {
            Err(e) => {
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Error while inserting root user: {}", e.Message).fg::<Red>()
                );
                return Ok(());
            }
            Ok(_) => {
                println!(
                    "{} - {} {}",
                    getCurrentTimeStr(),
                    format!("Inserted root user (Password: admin)").fg::<Green>(),
                    format!(
                        "Use this environment variable only when seting up the server!!! (set it to false when done)"
                    ).fg::<Red>()
                );
            }
        }
    }

    //Tries to find TLS keys for secure communication
    println!("{} - Finding TLS keys", getCurrentTimeStr());
    let mut certExists: bool = false;
    let mut sslBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("Error while creating ssl builder: ");
    if Path::new("config/key.pem").exists() && Path::new("config/cert.pem").exists() {
        println!(
            "{} - Found TLS keys (key.pem, cert.pem)",
            getCurrentTimeStr()
        );
        certExists = true;
        // load TLS keys
        // to create a self-signed temporary cert for testing:
        // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`

        match sslBuilder.set_private_key_file("config/key.pem", SslFiletype::PEM) {
            Ok(_) => {
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Opened key.pem").fg::<Green>()
                );
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
                println!(
                    "{} - {}",
                    getCurrentTimeStr(),
                    format!("Opened cert.pem").fg::<Green>()
                );
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

    let httpServer = HttpServer::new(move || {
        App::new()
            .app_data(PathConfig::default().error_handler(|err, _req| {
                let errorString = err.to_string();
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(PickUpError {
                        Code: PickUpErrorCode::BadRequest,
                        Message: errorString.to_string(),
                    }),
                )
                .into()
            }))
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                let errorString = err.to_string();
                error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(PickUpError {
                        Code: PickUpErrorCode::BadRequest,
                        Message: errorString.to_string(),
                    }),
                )
                .into()
            }))
            .app_data(web::Data::new(AppState {
                version: VERSION.to_string(),
                pepper: envPasswordPapper.clone(),
                pool: pool.clone(),
                hashingParameters: HashingParameters {
                    mem_cost: envMemCost.clone(),
                    time_cost: envTimeCost.clone(),
                    lanes: envLanes.clone(),
                },
            }))
            .wrap(Cors::default().allowed_origin("http://localhost:5173").allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"]).allowed_headers(&[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE, header::CONTENT_LENGTH]).supports_credentials())
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/docs/openapi.json", ApiDoc::openapi())
                    .config(Config::default().use_base_layout().filter(true)),
            )
            .service(index)
            .service(version)
            .service(controllers::Token::refresh::refresh)
            .service(controllers::Token::access::access)
            .service(
                web::scope("/user")
                    .service(controllers::User::userGet::getAllUsers)
                    .service(controllers::User::userGet::getUserById)
                    .service(controllers::User::userPost::postUser)
                    .service(controllers::User::userPatch::patchUser)
                    .service(controllers::User::userDelete::deleteUser),
            )
            .service(
                web::scope("/zipcode")
                    .service(controllers::ZipCode::zipcodeGet::getAllZipCodes)
                    .service(controllers::User::userGet::getAllUsers)
                    .service(controllers::ZipCode::zipcodeGet::getZipCodeById)
                    .service(controllers::ZipCode::zipcodePost::postZipCode)
                    .service(controllers::ZipCode::zipcodePatch::patchZipCode)
                    .service(controllers::ZipCode::zipcodeDelete::deleteZipCode),
            )
    });

    if certExists {
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            format!("Server starting with {}", envIpWithPort).fg::<Green>()
        );
        return httpServer
            .bind_openssl(&envIpWithPort, sslBuilder)
            .expect("Error while setting address with port/openssl: ")
            .run()
            .await;
    } else {
        println!(
            "{} - {}",
            getCurrentTimeStr(),
            format!("Server starting with {}", envIpWithPort).fg::<Green>()
        );
        return httpServer
            .bind(&envIpWithPort)
            .expect("Error while setting address with port: ")
            .run()
            .await;
    }
}
