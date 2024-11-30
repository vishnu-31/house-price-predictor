
use std::fs::File;
use std::io::Read;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::path::Path;
use aws_sdk_s3::config::Region;

pub async fn upload_file_to_s3(bucket_name: &str, local_file_path: &str, s3_location: &str) -> anyhow::Result<()> {


    let mut file = File::open(local_file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let region_provider = RegionProviderChain::first_try(Region::new("ap-south-1"))
        .or_default_provider();
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&shared_config); 

    let result =client
        .put_object()
        .bucket(bucket_name)
        .key(local_file_path)
        .body(ByteStream::from_path(local_file_path).await.unwrap())
        .send()
        .await
        .expect("Failed to upload the file");

    print!("upload successful. {:?} ", Some(result.e_tag()));

    print!("File {} uploaded to {} successfully", local_file_path, s3_location);

    Ok(())
}


///Download the model from S3 and return the path
pub  async fn download_model_from_s3(bucket_name: &str, s3_location: &str) -> anyhow::Result<String> {

    let region_provider = RegionProviderChain::first_try(Region::new("ap-south-1"))
        .or_default_provider();
    let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&shared_config); 

    // let bucket_name = "house-price-prediction-project";
    // let s3_location = "xgboost_model.bin";
    // It should be made to be given as input from cli to the cargo run command
    // for example: cargo run --bin train --bucket-name "house-price-prediction-project" \
    // --s3-location "xgboost_model.bin"

    let download_path = "downloaded_model.bin";

    let path = Path::new(download_path);
    if path.exists() {
        println!("File {} already exists. Skipping download", download_path);
        return Ok(download_path.to_string());
    }

    let result =client
        .get_object()
        .bucket(bucket_name)
        .key(s3_location)
        .send()
        .await?;

    let data = result.body.collect().await?.into_bytes();

    std::fs::write(download_path, data)?;
    print!("File {} downloaded to {} successfully", s3_location, download_path);
    Ok(download_path.to_string())

}