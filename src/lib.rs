use anyhow::Ok;
use polars::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
    // https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-01.parquet
    let nyc_url = "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-0";

    for n in 1..10 {
        let response = reqwest::blocking::get(format!("{}{}.parquet", nyc_url, n))?;
        let bytes = response.bytes()?;
        let nyc_data_path = format!("nyc_data_0{}.parquet", n);
        std::fs::write(nyc_data_path, bytes)?;
    }
    Ok("NYC data is downloaded for the current year".to_string())
}

pub fn train_test_split(
    df: &DataFrame,
    test_size_percent: f64,
) -> anyhow::Result<(DataFrame, DataFrame)> {
    let mut indices: Vec<usize> = (0..df.height()).collect();
    //Shuffle the indices
    let mut rng = thread_rng();

    indices.shuffle(&mut rng);

    let split_ind = (df.height() as f64 * (1.0 - test_size_percent)) as usize;

    let train_indices = indices[0..split_ind].to_vec();
    let test_indices = indices[split_ind..].to_vec();

    let train_indices_ca =
        UInt32Chunked::from_vec("", train_indices.iter().map(|&i| i as u32).collect());
    let test_indices_ca =
        UInt32Chunked::from_vec("", test_indices.iter().map(|&i| i as u32).collect());

    let train_df = df.take(&train_indices_ca)?;
    let test_df = df.take(&test_indices_ca)?;
    println!("Train DF shape: {:?}", train_df.height());
    println!("Test DF shape: {:?}", test_df.height());
    Ok((train_df, test_df))
}