use actix_web::{get, post, App, HttpResponse, HttpServer, Responder,web};
use env_logger;
use log::info;
use serde;
use clap::Parser;
use house_price_predictor::aws::download_model_from_s3;
use house_price_predictor::model::{load_model,Model};
use dotenv;
use std::sync::Arc;

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




/// AppState will be shared across all my workers in actix web
struct AppState {
    model: Arc<Model>,
}
///Predict Endpoint
/// Accepts a JSON paylod with features and returns a prediction
#[post("/predict")]
async fn predict(
    data: web::Data<AppState>,
    payload: web::Json<PredictRequest>
    ) -> impl Responder {
    info!("Predict checkpoint called");
    let names = data.model.get_attribute_names().unwrap();
        println!("Model metatdata {:?} ", names);
    // println!("Payload: {}", payload);
    // let features: Float32Chunked = Vec![payload.crim, payload.zn, payload.indus, payload.chas, payload.nox, payload.rm, payload.age, payload.dis, payload.rad, payload.tax, payload.ptratio, payload.b, payload.lstat];
    // let prediction = data.model::predict(
    //     payload.crim, 
    //     payload.zn, 
    //     payload.indus, 
    //     payload.chas, 
    //     payload.nox, 
    //     payload.rm, 
    //     payload.age, 
    //     payload.dis, 
    //     payload.rad, 
    //     payload.tax, 
    //     payload.ptratio, 
    //     payload.b, 
    //     payload.lstat);
    HttpResponse::Ok().body("Prediction".to_string() )
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();
    dotenv::dotenv().ok();


    let model_path = download_model_from_s3(&args.s3_bucket_name, &args.key).await?;
    info!("starting API...");
    // let app_state = AppState {
    //     model: Arc::new(model),
    // };

    let _ = HttpServer::new(move|| {
            
        let model = load_model(&model_path).unwrap();

        let app_state = AppState {
            model: Arc::new(model)
        };
            App::new()
            .app_data(web::Data::new(app_state))
            .service(health)
            .service(predict)})
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
    // .expect("Failed to start server")
}

