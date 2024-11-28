
use actix_web::{App,HttpServer,Responder,HttpResponse,get};
use log::{info,error};
use env_logger;



/// Health check endpoint 
/// Returns a 200 Ok response when the api is running
/// with a message "I am healthy"
#[get("/health")]
async fn health() -> impl Responder {
    info!("Health checkpoint called");
    HttpResponse::Ok().body("I am healthy")
}


#[actix_web::main]
async fn main() -> anyhow::Result<()> {


    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting API...");

    let  _ =HttpServer::new(||{
        App::new()
        .service(health)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await;
    Ok(())
    // .expect("Failed to start server")
}