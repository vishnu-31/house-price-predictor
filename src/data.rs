use anyhow::Ok;
use polars::prelude::*;
use std;


pub fn download_csv_file() -> anyhow::Result<String> {
    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    let file_path = "boston_housing.csv";
    std::fs::write(file_path, bytes)?;
    Ok(file_path.to_string())
}

pub fn load_csv(file_path: &str) -> anyhow::Result<DataFrame> {
    let df = CsvReader::from_path(file_path)?.finish()?;

    println!(
        "Loaded with {} rows and {} columns",
        df.height(),
        df.width()
    );
    println!("{:?}", df.head(Some(5)));

    Ok(df)
}

pub fn download_nyc_data() -> anyhow::Result<String> {
    let nyc_url = "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-0";

    for n in 1..10 {
        let response = reqwest::blocking::get(format!("{}{}.parquet", nyc_url, n))?;
        let bytes = response.bytes()?;
        let nyc_data_path = format!("nyc_data_0{}.parquet", n);
        std::fs::write(nyc_data_path, bytes)?;
    }
    Ok("NYC data is downloaded for the current year".to_string())
}
