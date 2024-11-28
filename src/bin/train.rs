use anyhow::Ok;
use house_price_predictor::split_features_and_target;
use house_price_predictor::train_test_split;
use house_price_predictor::{download_csv_file, load_csv};
use house_price_predictor::triain_xgboost_model;
use house_price_predictor::upload_file_to_s3;
//use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("Starting training Script....");

    //# Download the csv file
    let saved_path = download_csv_file()?;
    println!("The path of CSV is {}", saved_path);

    // Load dataset into memory
    let df = load_csv(&saved_path)?;

    //split the data to training and testing sets
    let (train_df, test_df) = train_test_split(&df, 0.2)?;

    let (x_train, y_train) = split_features_and_target(&train_df)?;
    let (x_test, y_test) = split_features_and_target(&test_df)?;

    let path_to_model = triain_xgboost_model(&x_train, &y_train, &x_test ,&y_test)?;

    print!("model is saved in {}", path_to_model);

    upload_file_to_s3("house-price-prediction-project",&path_to_model,"model.bin")?;
    Ok(())
}
