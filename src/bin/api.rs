use actix_web::{get, post, App, HttpResponse, HttpServer, Responder,web};
use env_logger;
use log::info;
use serde::{Deserialize, Serialize};
use clap::Parser;
use house_price_predictor::aws::download_model_from_s3;
use house_price_predictor::model::{load_model,Model};
use dotenv;
use xgboost::DMatrix;
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

    #[arg(short, long, default_value_t = 8080)]
    port: u16
}

#[derive(Deserialize)]
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


#[derive(Serialize)]
struct PredictResponse {
    prediction: f32
}

fn convert_payload_to_dmatrix(payload: &PredictRequest) -> anyhow::Result<DMatrix> {
    let features: Vec<f32> = [
        payload.crim, 
        payload.zn, 
        payload.indus, 
        payload.chas, 
        payload.nox, 
        payload.rm, 
        payload.age, 
        payload.dis, 
        payload.rad, 
        payload.tax, 
        payload.ptratio, 
        payload.b, 
        payload.lstat].iter().map(|f| *f as f32).collect();
    let dmatrix = DMatrix::from_dense(&features, 1)?;

    Ok(dmatrix)
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
        
    // info!("Features sent by client {:?}", payload);

    let model = &data.model;
    let dmatrix_features = convert_payload_to_dmatrix(&payload.0).unwrap();


    let prediction = model.predict(&dmatrix_features).unwrap()[0];
    let prediction_response = PredictResponse {
        prediction: prediction,
    
    };
   web::Json(prediction_response)
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
        .bind(("127.0.0.1", args.port))?
        .run()
        .await;
    Ok(())
    // .expect("Failed to start server")
}

