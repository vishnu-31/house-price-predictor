use anyhow::Ok;
use house_price_predictor::model::split_features_and_target;
use house_price_predictor::model::train_test_split;
use house_price_predictor::data::{download_csv_file, load_csv};
use house_price_predictor::model::triain_xgboost_model;
use house_price_predictor::aws::upload_file_to_s3;
use tokio::runtime::Runtime;
use clap::{Parser,arg};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    s3_bucket_name: String,

    #[arg(short, long)]
    key: String,
}

fn main() -> anyhow::Result<()> {


    let args = Args::parse();
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

    let bucket_name = &args.s3_bucket_name;
    let s3_key = &args.key;
    let rt = Runtime::new()?;
    rt.block_on(async{
        upload_file_to_s3(bucket_name,&path_to_model,s3_key).await.unwrap();
    });
    Ok(())
}
