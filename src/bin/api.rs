use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use env_logger;
use log::info;
use serde;
use clap::Parser;
use house_price_predictor::aws::download_model_from_s3;
use dotenv;
/// Health check endpoint
/// Returns a 200 Ok response when the api is running
/// with a message "I am healthy"
#[get("/health")]
async fn health() -> impl Responder {
    info!("Health checkpoint called");
    HttpResponse::Ok().body("I am healthy")
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    s3_bucket_name: String,

    #[arg(short, long)]
    key: String,
}

#[derive(serde::Deserialize)]
struct PredictRequest {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    b: f64,
    lstat: f64
}
///Predict Endpoint
/// Accepts a JSON paylod with features and returns a prediction
#[post("/predict")]
async fn predict(payload: actix_web::web::Json<PredictRequest>) -> impl Responder {
    info!("Predict checkpoint called");
    // println!("Payload: {}", payload);
    HttpResponse::Ok().body("Prediction")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();
    dotenv::dotenv().ok();


    let model_path = download_model_from_s3(&args.s3_bucket_name, &args.key).await.unwrap();
    println!("model downloaded to {}",model_path);
    info!("starting API...");

    let _ = HttpServer::new(|| App::new().service(health).service(predict))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
    // .expect("Failed to start server")
}

