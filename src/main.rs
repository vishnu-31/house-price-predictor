use anyhow::Ok;
use house_price_predictor::train_test_split;
use house_price_predictor::{download_csv_file, load_csv};
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

    Ok(())
}
